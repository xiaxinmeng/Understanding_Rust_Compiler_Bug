Rust
use std::slice;
use std::u32;

#[repr(packed)]
#[derive(Copy, Clone)]
struct Unaligned<T>(T);

impl<T> Unaligned<T> {
    fn get(self) -> T { self.0 }
}

fn bytes_to_words(b: &[u8]) -> &[Unaligned<u32>] {
    unsafe { slice::from_raw_parts(b.as_ptr() as *const Unaligned<u32>, b.len() / 4) }
}

fn main() {
        let values = String::from("fedcba98765432100123456789abcdef");
        let bytes = values.as_bytes();
        let mut words = &bytes_to_words(&bytes[1..]);
        let index = 1;
        let position = u32::from_le(words[index].get());
}
