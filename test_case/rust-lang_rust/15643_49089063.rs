 rust
extern crate std;
extern crate native;
extern crate debug;

use native::io::file::open;
use std::rt::rtio::{Open, Read};
use std::os::{MemoryMap, MapReadable, MapWritable, MapFd};

fn main() {
  let path = "test.bin";
  let file =
    match open(&path.to_c_str(), Open, Read) {
      Err(_) => { fail!("Something is terribly wrong!"); },
      Ok(f)  => { f },
    };
  let fmap = 
    match MemoryMap::new(4096u, [MapReadable, MapWritable, MapFd(file.fd())]) {
      Err(_) => { fail!("Something is terribly wrong!"); },
      Ok(f)  => { f },
    };
  println!("{:?}", fmap);
}
