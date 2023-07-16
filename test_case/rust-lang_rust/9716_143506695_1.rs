
hello.rs:10:20: 10:22 error: lifetime name `'a` shadows a lifetime name that is already in scope [E0496]
hello.rs:10     fn from_buffer<'a>(buffer: &'a[u8]) -> Buffer<'a> {
                               ^~
hello.rs:8:6: 8:8 note: shadowed lifetime `'a` declared here
hello.rs:8 impl<'a> BufferMe for Buffer<'a> {
                ^~
error: aborting due to previous error
