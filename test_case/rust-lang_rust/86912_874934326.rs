rust
#![feature(array_map)]

pub fn main() {
    (|| {
        [0u8; 128 * 1024].map(|e| e + 1);
        // Stack overflows, meaning the stack usage was more than 16 times the size of the array!
    })();
}
