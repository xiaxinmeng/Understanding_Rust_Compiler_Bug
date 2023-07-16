console
$ rustup-toolchain-install-master 4700e1188f66fdb4086b7593416b678b8fe935f4
downloading <https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rustc-builds/4700e1188f66fdb4086b7593416b678b8fe935f4/rustc-nightly-x86_64-apple-darwin.tar.xz>...
completed                                                                                                                                                                                                                                  
downloading <https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rustc-builds/4700e1188f66fdb4086b7593416b678b8fe935f4/rust-std-nightly-x86_64-apple-darwin.tar.xz>...
completed                                                                                                                                                                                                                                  
toolchain `4700e1188f66fdb4086b7593416b678b8fe935f4` is successfully installed!

$ rustc +4700e1188f66fdb4086b7593416b678b8fe935f4 1.rs
error[E0596]: cannot borrow field of immutable binding as mutable
 --> 1.rs:7:24
  |
7 |     if let (Some(Foo::Bar(ref mut val)), _) = (&arr.get(0), 0) {
  |                           ^^^^^^^^^^^ cannot mutably borrow field of immutable binding

error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
