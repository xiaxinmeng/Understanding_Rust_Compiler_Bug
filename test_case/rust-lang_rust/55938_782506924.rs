
$ rustdoc +stable --version
rustdoc 1.50.0 (cb75ad5db 2021-02-10)
$ rustdoc +stable feature.rs 
error[E0554]: `#![feature]` may not be used on the stable release channel
 --> feature.rs:1:1
  |
1 | #![feature(doc_cfg)]
  | ^^^^^^^^^^^^^^^^^^^^

error: Compilation failed, aborting rustdoc
