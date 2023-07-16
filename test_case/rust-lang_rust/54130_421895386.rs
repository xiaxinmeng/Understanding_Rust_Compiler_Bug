
$ rustc +nightly-2018-09-09 -vV                                                                                                                                 
rustc 1.30.0-nightly (0198a1ea4 2018-09-08)                                                                                                                                                                                                   
binary: rustc
commit-hash: 0198a1ea45e29af00d92423aa6d2ac876410c3f9
commit-date: 2018-09-08
host: x86_64-unknown-linux-gnu
release: 1.30.0-nightly                                                                                                                                                                                                                       
LLVM version: 8.0

$ rustc +nightly-2018-09-09 54130.rs                                                                                                                            
error[E0660]: malformed inline assembly                                                                                                                                                                                                       
 --> 54130.rs:4:13
  |
4 |     unsafe {asm!("", :"={rax"(rax))};
  |             ^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0660`. 

$ rustc +nightly-2018-09-09 -O 54130.rs                                                                                                                         
error[E0660]: malformed inline assembly                                                                                                                                                                                                       
 --> 54130.rs:4:13
  |
4 |     unsafe {asm!("", :"={rax"(rax))};
  |             ^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0660`.
