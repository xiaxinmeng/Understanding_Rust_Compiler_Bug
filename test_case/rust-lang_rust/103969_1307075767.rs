plain
   Compiling toml v0.5.9
error[E0425]: cannot find value `channel` in this scope
    --> config.rs:1679:43
     |
1679 |         let filename = format!("rust-src-{channel}.tar.xz");
     |
help: consider importing this function
     |
6    | use std::sync::mpsc::channel;
