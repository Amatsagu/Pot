use super::super::errors::DatabaseError;
use bincode::deserialize;
use bincode::serialize;
use serde_json::from_value;
use serde_json::Value;
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

pub struct Chunk {
    mutex_reader: Mutex<BufReader<File>>,
    mutex_writer: Mutex<BufWriter<File>>,
    chunk_id: u8,
}

pub struct ChunkOptions {
    base_dir: String,
    chunk_id: u8,
}

impl Chunk {
    pub fn new(opt: ChunkOptions) -> IOResult<Self> {
        let mut open_options = OpenOptions::new();
        let file_handle = open_options
            .read(true)
            .write(true)
            .create(true)
            .open(format!("{}/chunks/{}", opt.base_dir, opt.chunk_id))?;

        Ok(Self {
            mutex_reader: Mutex::new(BufReader::new(file_handle)),
            mutex_writer: Mutex::new(BufWriter::new(file_handle)),
            chunk_id: opt.chunk_id,
        })
    }

    pub async fn get<T: From<Value>>(self, ptr: u64, length: usize) -> Result<T, DatabaseError> {
        let r: Result<Value, DatabaseError> = task::spawn(async move {
            let mut read_handle = self.mutex_reader.lock().unwrap();
            read_handle.seek(SeekFrom::Start(ptr));
            let mut buff = Vec::with_capacity(length);
            read_handle.read(&mut buff)?;

            Ok(bincode::deserialize(&buff)?)
        })
        .await
        .unwrap();

        r.map(|v| v.into())
    }
}
