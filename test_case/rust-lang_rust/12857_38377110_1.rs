
% rustc --cfg hide_fails iss12857-c.rs
iss12857-c.rs:25:19: 25:22 error: `foo` does not live long enough
iss12857-c.rs:25         Hide::new(foo.as_slice())
                                   ^~~
iss12857-c.rs:22:16: 31:2 note: reference must be valid for the block at 22:15...
iss12857-c.rs:22 pub fn main () {
iss12857-c.rs:23     let bar : Hide = {
iss12857-c.rs:24         let foo = ~[1, 2, 3, 4, 5];
iss12857-c.rs:25         Hide::new(foo.as_slice())
iss12857-c.rs:26     }; // does not typecheck: `foo` does not live long enough
iss12857-c.rs:27 
                 ...
iss12857-c.rs:23:22: 26:6 note: ...but borrowed value is only valid for the block at 23:21
iss12857-c.rs:23     let bar : Hide = {
iss12857-c.rs:24         let foo = ~[1, 2, 3, 4, 5];
iss12857-c.rs:25         Hide::new(foo.as_slice())
iss12857-c.rs:26     }; // does not typecheck: `foo` does not live long enough
error: aborting due to previous error
% rustc --cfg hide_works iss12857-c.rs
% ./iss12857-c
bar.slice = [9, 9, 9, 9, 9], should reject (was [1, 2, 3, 4, 5])
% 
