
test.rs:10:5: 14:6 error: method `from_buffer` has an incompatible type: expected concrete lifetime, but found bound lifetime parameter &'a 
test.rs:10     fn from_buffer<'b>(buffer: &'b[u8]) -> Buffer<'b> {
test.rs:11         Buffer {
test.rs:12             buffer: buffer
test.rs:13         }
test.rs:14     }
test.rs:10:55: 14:6 note: expected concrete lifetime is the lifetime &'c  as defined on the block at 10:54
test.rs:10     fn from_buffer<'b>(buffer: &'b[u8]) -> Buffer<'b> {
test.rs:11         Buffer {
test.rs:12             buffer: buffer
test.rs:13         }
test.rs:14     }
