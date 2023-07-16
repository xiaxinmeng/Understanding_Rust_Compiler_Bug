
error: unexpected compiler error or warning: '/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/src/test/compile-fail/issue-3036.rs:16:0: 16:1 error: expected `;`, found `}`'
command: x86_64-unknown-linux-gnu/stage2/bin/rustc /home/rustbuild/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/src/test/compile-fail/issue-3036.rs -o x86_64-unknown-linux-gnu/test/compile-fail/issue-3036.stage2-x86_64-unknown-linux-gnu -L x86_64-unknown-linux-gnu/test/compile-fail -L x86_64-unknown-linux-gnu/test/compile-fail/issue-3036.stage2-x86_64-unknown-linux-gnu.libaux --target=x86_64-unknown-linux-gnu
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/src/test/compile-fail/issue-3036.rs:16:0: 16:1 error: expected `;`, found `}`
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/src/test/compile-fail/issue-3036.rs:16 } //~ ERROR: expected `;` but found `}`
                                                                                                          ^
