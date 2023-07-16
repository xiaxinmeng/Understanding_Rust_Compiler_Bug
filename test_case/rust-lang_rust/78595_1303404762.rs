rust
// use std::io::Read;

fn make_reader() -> Box<dyn std::io::Read + 'static> {
    let s = "hello world";
    Box::new(s.as_bytes()) 
}   

fn main() { 
    let mut buf = Vec::new();
    
    make_reader().take(100).read_to_end(&mut buf).unwrap();
    
    println!("{:?}", buf);
}
