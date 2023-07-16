
foo.rs:23:15: 23:21 error: borrowed value does not live long enough
foo.rs:23     print_name(&thing as &Debuggable);
                         ^~~~~~
note: borrowed pointer must be valid for the static lifetime...
foo.rs:21:10: 24:1 note: ...but borrowed value is only valid for the block at 21:10
foo.rs:21 fn main() {
foo.rs:22     let thing = Thing::new();
foo.rs:23     print_name(&thing as &Debuggable);
foo.rs:24 }
error: aborting due to previous error
