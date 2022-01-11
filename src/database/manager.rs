use super::chunk::Chunk;
use super::header::Header;

pub struct Database {
    header: Header,
    chunks: Vec<Chunk>,
}

impl Database {
    pub fn new() -> Self {}
}

