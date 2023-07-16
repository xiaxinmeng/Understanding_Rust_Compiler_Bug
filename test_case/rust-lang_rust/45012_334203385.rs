rust
#[macro_use]
extern crate serde_derive;
extern crate bincode;

use std::io::Write;
use std::{io, ptr};

use bincode::{serialize, deserialize, Infinite};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Entity {
    x: f64,
    y: f64,
    z: f64,
    o: u64
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct World(Vec<Entity>);

struct UnsafeVecWriter<'a>(&'a mut Vec<u8>);

impl<'a> Write for UnsafeVecWriter<'a> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unsafe {
            let old_len = self.0.len();
            self.0.set_len(old_len + buf.len());
            ptr::copy_nonoverlapping(buf.as_ptr(), self.0.as_mut_ptr().offset(old_len as isize), buf.len());
        }
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}

struct SizeCounter(usize);

impl<'a> Write for SizeCounter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0 += buf.len();
        Ok(buf.len())
    }
    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}

#[inline(never)]
fn make_bytes(vec: &mut Vec<u8>, e: &Entity) {
    let mut size = SizeCounter(0);
    bincode::serialize_into(&mut size,e , Infinite).unwrap();
    vec.reserve(size.0);
    //vec.reserve(bincode::serialized_size(&e) as usize);

    bincode::serialize_into(&mut UnsafeVecWriter(vec), e, Infinite).unwrap();
}

#[inline(never)]
fn slow_make_bytes(vec: &mut Vec<u8>, e: &Entity) {
    bincode::serialize_into(vec, e, Infinite).unwrap();
}

fn main() {
    let world = Entity { x: 0.0, y: 4.0, z: 5.0, o: 0 };

    let mut encoded = Vec::new();
    make_bytes(&mut encoded, &world);
    slow_make_bytes(&mut encoded, &world);

    // 8 bytes for the length of the vector, 4 bytes per float.
    //assert_eq!(encoded.len(), 8 + 4 * 4);

    let decoded: Entity = deserialize(&encoded[..]).unwrap();

    assert_eq!(world, decoded);
}
