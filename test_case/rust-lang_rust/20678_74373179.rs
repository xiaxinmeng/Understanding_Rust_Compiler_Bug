
use std::io::{Cursor,Seek,SeekFrom,BufRead};
fn main() {
        let mut mr = Cursor::new(vec![]);
            mr.consume((1u64 << 63) as usize);
                println!("{:?}", mr.seek(SeekFrom::Current(0)));
}
