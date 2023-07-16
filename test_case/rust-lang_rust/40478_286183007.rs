
use std::io::prelude::*;
use std::io::Cursor;

fn main() {
    let mut s = Cursor::new(vec![]);
    
    s.write_all(&[1,2,3]).unwrap();
    assert_eq!(s.get_ref().len(), 3);
    
    s.seek(::std::io::SeekFrom::Current(-3)).unwrap();
    
    let mut b = [0u8];
    let r = s.read_exact(&mut b);
    println!("{:?}", b);
    assert!(r.is_ok());
}
