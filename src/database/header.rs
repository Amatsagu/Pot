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

pub struct Header {}

pub struct HeaderOptions {
    pub directory_name: String,
    pub key_length: u8,
    pub chunk_count: u8,
}
