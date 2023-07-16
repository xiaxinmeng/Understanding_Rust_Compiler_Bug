

error: unexpected compiler error or warning: '/home/rustbuild/src/rust-buildbot/slave/auto-linux-32-opt/build/src/test/compile-fail/issue-3099-b.rs:13:0: 13:12 error: duplicate definition of module `a`'
command: i686-unknown-linux-gnu/stage2/bin/rustc /home/rustbuild/src/rust-buildbot/slave/auto-linux-32-opt/build/src/test/compile-fail/issue-3099-b.rs -o i686-unknown-linux-gnu/test/compile-fail/issue-3099-b.stage2-i686-unknown-linux-gnu -L i686-unknown-linux-gnu/test/compile-fail -L i686-unknown-linux-gnu/test/compile-fail/issue-3099-b.libaux -O --target=i686-unknown-linux-gnu
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/home/rustbuild/src/rust-buildbot/slave/auto-linux-32-opt/build/src/test/compile-fail/issue-3099-b.rs:13:0: 13:12 error: duplicate definition of module `a`
/home/rustbuild/src/rust-buildbot/slave/auto-linux-32-opt/build/src/test/compile-fail/issue-3099-b.rs:13 pub mod a {} //~ ERROR duplicate definition of type `a`
                                                                                                         ^~~~~~~~~~~~
/home/rustbuild/src/rust-buildbot/slave/auto-linux-32-opt/build/src/test/compile-fail/issue-3099-b.rs:11:0: 11:12 note: first definition of module `a` here
/home/rustbuild/src/rust-buildbot/slave/auto-linux-32-opt/build/src/test/compile-fail/issue-3099-b.rs:11 pub mod a {}
                                                                                                         ^~~~~~~~~~~~
error: aborting due to previous error

------------------------------------------
