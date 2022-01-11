use serde::ser::Serialize;
use serde::ser::SerializeStruct;
use serde::ser::Serializer;

pub struct Key {
    value: String,
    chunk_ptr: u8,
    key_ptr: u64,
    length: u64,
}

impl Serialize for Key {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Key", 4)?;
        state.serialize_field("value", &self.value)?;
        state.serialize_field("chunk_ptr", &self.chunk_ptr)?;
        state.serialize_field("key_ptr", &self.key_ptr)?;
        state.serialize_field("length", &self.length)?;
        state.end()
    }
}
