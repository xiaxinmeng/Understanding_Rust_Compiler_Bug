rust
#![feature(nll)]

struct Packet<'a> {
    b: &'a [u8],
}

impl<'a> Packet<'a> {
    fn new(b: &'a [u8]) -> Option<Self> {
        Some(Packet { b: b })
    }
}

struct PacketReader {
    bytes: Vec<u8>,
}

impl PacketReader {
    fn poll<'a>(&'a mut self) -> Packet<'a> {
        loop {
            if let Some(p) = Packet::new(&self.bytes[..]) {
                return p;
            }

            // we need to read some more
            self.bytes.truncate(0);
        }
    }
}
