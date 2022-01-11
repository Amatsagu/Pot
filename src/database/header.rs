use std::collections::HashMap;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::io::Result as IOResult;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::sync::Mutex;
use tokio::task;

pub struct Header {
    mutex_reader: Mutex<BufReader<File>>,
    mutex_writer: Mutex<BufWriter<File>>,
    key_count: u64,
    key_length: u8,
    chunk_count: u8,
    keys: HashMap<String, usize>,
}

pub struct HeaderOptions {
    pub base_dir: String,
    pub key_length: u8,
    pub chunk_count: u8,
}

impl Header {
    fn load_header(opt: HeaderOptions) -> IOResult<Self> {
        let mut open_options = OpenOptions::new();
        let file_handle = open_options
            .read(true)
            .write(true)
            .create(false)
            .open(format!("{}/header", opt.base_dir))?;
        file_handle.seek(SeekFrom::Start(0));
        let bytes = [0; 2];
        let key_count_bytes = [0; 8];
        file_handle.read(&mut bytes)?;
        file_handle.read(&mut key_count_bytes)?;
        let key_length = bytes[0];
        let chunk_count = bytes[1];
        let key_count = u64::from_be_bytes(key_count_bytes);
        let mut header = Self {
            mutex_reader: Mutex::new(BufReader::new(file_handle)),
            mutex_writer: Mutex::new(BufWriter::new(file_handle)),
            key_length,
            key_count,
            chunk_count,
            keys: HashMap::new(),
        };

        for i in 0..key_count as usize {
            let buff = Vec::with_capacity(key_length as usize);
            let read = file_handle.read(&mut buff)?;
            if read != key_length as usize {
                break;
            }
            let key = String::from_utf8_lossy(&buff).to_string();
            header.keys.insert(key, i);
        }

        Ok(header)
    }

    fn create_header(opt: HeaderOptions) -> IOResult<Self> {
        let mut open_options = OpenOptions::new();
        let file_handle = open_options
            .read(true)
            .write(true)
            .create(true)
            .open(format!("{}/header", opt.base_dir))?;

        file_handle.write(&[0; 10]);
        file_handle.flush();

        Ok(Self {
            mutex_reader: Mutex::new(BufReader::new(file_handle)),
            mutex_writer: Mutex::new(BufWriter::new(file_handle)),
            key_count: 0,
            key_length: 0,
            chunk_count: 0,
            keys: HashMap::new(),
        })
    }

    pub fn new(opt: HeaderOptions) -> IOResult<Self> {
        let header = match Header::load_header(opt) {
            Ok(v) => v,
            Err(_) => Header::create_header(opt)?,
        };

        Ok(header)
    }

    pub async fn set(mut self, key: String) -> bool {
        if !self.keys.contains_key(&key) {
            return false;
        }
        self.keys.insert(key.clone(), self.key_count as usize);
        let io_result: IOResult<()> = task::spawn(async move {
            let mut writer = self.mutex_writer.lock().unwrap();
            let ptr = self.key_count * self.key_length as u64;

            writer.seek(SeekFrom::Start(ptr));
            writer.write(&key.as_bytes())?;
            writer.flush()?;
            Ok(())
        })
        .await
        .unwrap();

        self.key_count += 1;

        true
    }

    pub fn get(&self, key: String) -> Option<(u8, usize)> {
        let v = *self.keys.get(&key)?;

        let chunk = (v % (self.chunk_count as usize)) as u8;
        let ptr = v / self.chunk_count as usize;

        Some((chunk, ptr))
    }

    #[inline]
    pub fn has(&self, key: String) -> bool {
        self.keys.contains_key(&key)
    }

    pub async fn remove(mut self, key: String) -> bool {
        let index: u64 = match self.keys.get(&key) {
            Some(v) => *v as u64,
            None => return false,
        };

        let ptr = index * self.key_count + 10;
        let next_ptr = ptr * self.key_count;
        self.key_count -= 1;
        let io_result: IOResult<()> = task::spawn(async move {
            let mut reader = self.mutex_reader.lock().unwrap();
            let mut writer = self.mutex_writer.lock().unwrap();

            let mut buff: Vec<u8> = Vec::new();

            reader.seek(SeekFrom::Start(next_ptr));
            reader.read_to_end(&mut buff);

            writer.seek(SeekFrom::Start(ptr));
            writer.write_all(&buff)?;
            writer.seek(SeekFrom::Start(2));
            writer.write_all(&self.key_count.to_be_bytes())?;
            writer.flush()?;

            Ok(())
        })
        .await
        .unwrap();
        io_result.unwrap();

        true
    }
}
