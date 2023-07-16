 rust
<anon>:6:46: 6:47 error: `r` does not live long enough
<anon>:6     let x : &mut Debug = Box::deref_mut(&mut r);
                                                      ^
<anon>:4:11: 7:2 note: reference must be valid for the block at 4:10...
<anon>:4 fn main() {
<anon>:5     let mut r: Box<Debug> = Box::new(1);
<anon>:6     let x : &mut Debug = Box::deref_mut(&mut r);
<anon>:7 }
<anon>:5:41: 7:2 note: ...but borrowed value is only valid for the block suffix following statement 0 at 5:40
<anon>:5     let mut r: Box<Debug> = Box::new(1);
<anon>:6     let x : &mut Debug = Box::deref_mut(&mut r);
<anon>:7 }
error: aborting due to previous error
playpen: application terminated with error code 101
Compilation failed.
