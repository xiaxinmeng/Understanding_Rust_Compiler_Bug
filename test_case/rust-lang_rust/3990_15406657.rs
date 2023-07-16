
% rustc /tmp/foo.rs 
/tmp/foo.rs:3:16: 3:20 error: structure has no field named `y`
/tmp/foo.rs:3 fn main() { S { y: 1 }; }
                              ^~~~
/tmp/foo.rs:3:12: 3:13 error: missing field: `x`
/tmp/foo.rs:3 fn main() { S { y: 1 }; }
                          ^
error: internal compiler error: no type for node 8: expr 1 (id=8) in fcx 7fc245a679a0
