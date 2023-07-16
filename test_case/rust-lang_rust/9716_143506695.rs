
hello.rs:10:5: 14:6 error: method `from_buffer` has an incompatible type for trait:
 expected bound lifetime parameter 'a,
    found concrete lifetime [E0053]
hello.rs:10     fn from_buffer<'b>(buffer: &'b[u8]) -> Buffer<'b> {
hello.rs:11         Buffer {
hello.rs:12             buffer: buffer
hello.rs:13         }
hello.rs:14     }
hello.rs:10:5: 14:6 help: run `rustc --explain E0053` to see a detailed explanation
error: aborting due to previous error
