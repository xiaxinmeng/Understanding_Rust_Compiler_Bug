
use std::io::{MemReader, EndOfFile, IoResult};

fn read_block<'a>(r: &mut Reader, buf: &'a mut [u8]) -> IoResult<&'a [u8]> {
    match r.read(buf) {
        Ok(len) => Ok(buf.slice_to(len)),
        Err(err) => {
            if err.kind == EndOfFile {
                Ok(buf.slice_to(0))
            } else {
                Err(err)
            }
        }
    }
}

fn main() {
    let mut buf = [0u8, ..2];
    let mut reader = MemReader::new(~[67u8, ..10]);
    let mut block = read_block(&mut reader, buf);
    loop {
        //process block
        block = read_block(&mut reader, buf); //error here
}
