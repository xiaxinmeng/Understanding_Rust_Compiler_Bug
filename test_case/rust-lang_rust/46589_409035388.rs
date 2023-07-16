rust
#![feature(nll)]

fn main() {}

pub struct Decoder {
    buf_read: BufRead,
}

impl Decoder {
    pub fn next<'a>(&'a mut self) -> &'a str {
        loop {
            let buf = self.buf_read.fill_buf();
            if let Some(s) = decode(buf) {
                return s
            }
            // loop to get more input data

            // At this point `buf` is not used anymore.
            // With NLL I would expect the borrow to end here,
            // such that `self.buf_read` is not borrowed anymore
            // by the time we start the next loop iteration.
        }
    }
}

struct BufRead;

impl BufRead {
    fn fill_buf(&mut self) -> &[u8] { unimplemented!() }
}

fn decode(_: &[u8]) -> Option<&str> { unimplemented!() }
