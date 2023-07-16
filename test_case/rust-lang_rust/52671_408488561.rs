
error[E0499]: cannot borrow `promoted` as mutable more than once at a time
  --> src/util/io_util.rs:45:47
   |
45 |         slice = &mut mem::replace(&mut slice, &mut [])[n..];
   |                                               ^^^^^^^ mutable borrow starts here in previous iteration of loop
   |
note: borrowed value must be valid for the anonymous lifetime #2 defined on the function body at 39:1...
  --> src/util/io_util.rs:39:1
   |
39 | / pub fn read_fill(r: &mut R, mut slice: &mut [u8]) -> io::Result<()> {
40 | |     while !slice.is_empty() {
41 | |         let n = r.read(slice)?;
42 | |         if n == 0 {
...  |
47 | |     Ok(())
48 | | }
   | |_^

error: aborting due to previous error
