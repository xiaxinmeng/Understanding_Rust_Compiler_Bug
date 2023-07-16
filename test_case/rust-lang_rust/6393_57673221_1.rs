
/path/to/mylib/src/buffer.rs:168:13: 168:17 error: cannot borrow `*self` as mutable more than once at a time
/path/to/mylib/src/buffer.rs:168             self.top_up()
                                                        ^~~~
/path/to/mylib/src/buffer.rs:160:33: 160:43 note: previous borrow of `*self.input` occurs here; the mutable borrow prevents subsequent moves, borrows, or modification of `*self.input` until the borrow ends
/path/to/mylib/src/buffer.rs:160                 let read = try!(self.input.fill_buf());
                                                                            ^~~~~~~~~~
<std macros>:1:1: 3:2 note: in expansion of try!
/path/to/mylib/src/buffer.rs:160:28: 160:56 note: expansion site
/path/to/mylib/src/buffer.rs:170:6: 170:6 note: previous borrow ends here
/path/to/mylib/src/buffer.rs:149     fn fill_buf<'a>(&'a mut self) -> IoResult<&'a [u8]> {
...
/path/to/mylib/src/buffer.rs:170     }
