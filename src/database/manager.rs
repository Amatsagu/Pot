use super::chunk::Chunk;
use super::header::Header;

pub struct Database {
    header: Header,
    chunks: Vec<Chunk>,
}

impl Database {
    pub fn new() -> Self {}
}

async fn ree() {
    let mut h = Header::new().unwrap();
    h.set("string".to_string());
    h.set("ree".to_string());
}
