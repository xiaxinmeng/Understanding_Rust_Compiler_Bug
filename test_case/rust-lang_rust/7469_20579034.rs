
error: unexpected compiler error or warning: '/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-opt/build/src/test/compile-fail/liveness-dead.rs:20:4: 20:10 warning: unnecessary `copy`; this value is implicitly copyable [-W copy-implicitly-copyable (default)]'
command: x86_64-unknown-linux-gnu/stage2/bin/rustc /home/rustbuild/src/rust-buildbot/slave/auto-linux-64-opt/build/src/test/compile-fail/liveness-dead.rs -o x86_64-unknown-linux-gnu/test/compile-fail/liveness-dead.stage2-x86_64-unknown-linux-gnu -L x86_64-unknown-linux-gnu/test/compile-fail -L x86_64-unknown-linux-gnu/test/compile-fail/liveness-dead.libaux -O --target=x86_64-unknown-linux-gnu
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-opt/build/src/test/compile-fail/liveness-dead.rs:20:4: 20:10 warning: unnecessary `copy`; this value is implicitly copyable [-W copy-implicitly-copyable (default)]
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-opt/build/src/test/compile-fail/liveness-dead.rs:20     copy x;
                                                                                                              ^~~~~~
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-opt/build/src/test/compile-fail/liveness-dead.rs:18:12: 18:13 error: value assigned to `x` is never read
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-opt/build/src/test/compile-fail/liveness-dead.rs:18     let mut x = 3; //~ ERROR: value assigned to `x` is never read
                                                                                                                      ^
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-opt/build/src/test/compile-fail/liveness-dead.rs:11:7: 11:22 note: lint level defined here
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-opt/build/src/test/compile-fail/liveness-dead.rs:11 #[deny(dead_assignment)];
                                                                                                                 ^~~~~~~~~~~~~~~
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-opt/build/src/test/compile-fail/liveness-dead.rs:25:4: 25:10 warning: unnecessary `copy`; this value is implicitly copyable [-W copy-implicitly-copyable (default)]
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-opt/build/src/test/compile-fail/liveness-dead.rs:25     copy x;
                                                                                                              ^~~~~~
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-opt/build/src/test/compile-fail/liveness-dead.rs:26:4: 26:5 error: value assigned to `x` is never read
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-opt/build/src/test/compile-fail/liveness-dead.rs:26     x = 4; //~ ERROR: value assigned to `x` is never read
                                                                                                              ^
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-opt/build/src/test/compile-fail/liveness-dead.rs:11:7: 11:22 note: lint level defined here
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-opt/build/src/test/compile-fail/liveness-dead.rs:11 #[deny(dead_assignment)];
                                                                                                                 ^~~~~~~~~~~~~~~
error: aborting due to 2 previous errors

------------------------------------------

rust: task failed at 'explicit failure', /home/rustbuild/src/rust-buildbot/slave/auto-linux-64-opt/build/src/compiletest/runtest.rs:725
