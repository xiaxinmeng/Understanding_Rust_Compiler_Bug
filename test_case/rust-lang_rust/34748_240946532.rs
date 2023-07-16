
   Compiling d2dpush v0.1.0 (file:///home/whs/apps/d2dpush)
src/d2dregistry.rs:4:27: 4:39 warning: unused import, #[warn(unused_imports)] on by default
src/d2dregistry.rs:4 use serde_json::builder::{ArrayBuilder, ObjectBuilder};
                                               ^~~~~~~~~~~~
<d2dpush macros>:3:1: 6:65 error: expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce` [E0525]
<d2dpush macros>:3 | req : & mut Request | {
                   ^
<d2dpush macros>:1:21: 1:77 note: in this expansion of json_handler_with_error! (defined in <d2dpush macros>)
src/main.rs:33:13: 33:65 note: in this expansion of json_handler_with_error! (defined in <d2dpush macros>)
src/main.rs:29:18: 55:6 note: in this expansion of router! (defined in <router macros>)
<router macros>:5:9: 31:27 note: the requirement to implement `Fn` derives from here
src/main.rs:29:18: 55:6 note: in this expansion of router! (defined in <router macros>)
error: aborting due to previous error
error: Could not compile `d2dpush`.
