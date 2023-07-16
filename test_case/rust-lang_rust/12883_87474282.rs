 rust
//file: abc.rs
use std::io::{BufStream, Read, Write};
use std::fs::File;
use std::path::Path;

fn main(){
    let file = File::open( &Path::new("message.txt")).unwrap();
    let mut stream = BufStream::new(file);
    stream.write("hello,world".as_bytes() );
    stream.flush();
    return;
    let mut buf = &mut [0; 100];
    match stream.read(buf){
        Ok(nread) => println!("Read {} bytes",nread),
        Err(e) => panic!("error reading :{}",e)
    }
}
