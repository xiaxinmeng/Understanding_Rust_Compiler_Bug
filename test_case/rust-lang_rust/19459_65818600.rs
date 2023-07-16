 Rust
struct Encoder {
    buffer: [u8, ..512],
}
impl Encoder {
    fn new_message(&mut self) -> MessageEncoder {
        MessageEncoder {
            writer: BufWriter::new(&mut self.buffer),
        }
    }
}

struct MessageEncoder {
    writer: BufWriter,
}
impl MessageEncoder {
    fn add_update(&mut self, update: &[u8]) -> bool {
        // write update into the internal buffer using the BufWriter
        // return a bool indicating whether the update could be written
        // note: If the update doesn't fit in the buffer, BufWriter returns an error and writes nothing. This fits my use case perfectly
    }
    fn encode(self, message_buffer: &mut [u8]) -> IoResult<&[u8]> {
        // copy message from internal buffer into the caller-provided buffer
        // return slice of caller-provided buffer, that covers exactly what we've written into it
        // this is where I need BufWriter::into_slice
    }
}
