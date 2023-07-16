 rust
a.rs:11:41: 11:47 error: borrowed value does not live long enough
a.rs:11     simplified_map_ref(x.borrow(), |r| &r.ok());
                                                ^~~~~~
note: in expansion of closure expansion
a.rs:11:36: 11:47 note: expansion site
a.rs:11:40: 11:47 note: reference must be valid for the anonymous lifetime #1 defined on the block at 11:39...
a.rs:11     simplified_map_ref(x.borrow(), |r| &r.ok());
                                               ^~~~~~~
a.rs:11:40: 11:47 note: ...but borrowed value is only valid for the block at 11:39
a.rs:11     simplified_map_ref(x.borrow(), |r| &r.ok());
                                               ^~~~~~~
error: aborting due to previous error
