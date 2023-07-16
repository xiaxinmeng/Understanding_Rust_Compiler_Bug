
rustc: x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/librustc_llvm
rustc: x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/bin/rustbook
rustc: x86_64-apple-darwin/stage1/lib/rustlib/x86_64-apple-darwin/bin/rustdoc
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/driver/driver.rs:15:14: 15:23 warning: obsolete syntax: "crate-name"
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/driver/driver.rs:15 extern crate "rustdoc" as this;
                                                                                                    ^~~~~~~~~
note: use an identifier not in quotes instead
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/driver/driver.rs:18:14: 18:28 warning: obsolete syntax: "crate-name"
/Users/rustbuild/src/rust-buildbot/slave/auto-mac-64-opt/build/src/driver/driver.rs:18 extern crate "rustc_driver" as this;
                                                                                                    ^~~~~~~~~~~~~~
cp: x86_64-apple-darwin/stage2/lib/librustdoc
rustc: x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libterm
rustc: x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libserialize
cp: x86_64-apple-darwin/stage2/bin/rustbook
cp: x86_64-apple-darwin/stage2/bin/rustdoc
rustbook: doc/book/index.html
error: No such file or directory (os error 2)
make: *** [doc/book/index.html] Error 101
make: *** Waiting for unfinished jobs....
program finished with exit code 2
elapsedTime=2077.192005
