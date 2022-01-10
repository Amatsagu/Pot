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

const KEY_COUNT_PTR: SeekFrom = SeekFrom::Start(2);

pub struct Header {
    mutex_reader: Mutex<BufReader<File>>,
    mutex_writer: Mutex<BufWriter<File>>,
    key_count: u64,
    key_length: u8,
    chunk_count: u8,
    keys: HashMap<String, usize>,
}

pub struct HeaderOptions {
    pub directory_name: String,
    pub key_length: u8,
    pub chunk_count: u8,
}

impl Header {
    pub fn new(opt: HeaderOptions) -> IOResult<Self> {
        Ok(Self)
    }

    pub async fn set(self, key: String) -> bool {
        true
    }

    pub fn get(&self, key: String) -> Option<(u8, usize)> {
        None
    }

    pub async fn remove(self, key: String) -> bool {
        true
    }
}
