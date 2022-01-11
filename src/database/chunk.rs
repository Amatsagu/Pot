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
        let file_handle = open_options.read(true).write(true).create(true).open(format!("{}/chunks/{}", opt.base_dir, opt.chunk_id))
    }
}
