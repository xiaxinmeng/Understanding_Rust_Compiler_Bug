
error: error pattern ' Unit-like struct construction is written with no trailing `{ }`' not found!
command: x86_64-unknown-linux-gnu/stage2/bin/rustc /home/rustbuild/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/src/test/compile-fail/struct-no-fields-4.rs -o x86_64-unknown-linux-gnu/test/compile-fail/struct-no-fields-4.stage2-x86_64-unknown-linux-gnu -L x86_64-unknown-linux-gnu/test/compile-fail -L x86_64-unknown-linux-gnu/test/compile-fail/struct-no-fields-4.stage2-x86_64-unknown-linux-gnu.libaux --target=x86_64-unknown-linux-gnu
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/src/test/compile-fail/struct-no-fields-4.rs:15:32: 15:33 error: unit-like struct construction is written with no trailing `{ }`
/home/rustbuild/src/rust-buildbot/slave/auto-linux-64-nopt-t/build/src/test/compile-fail/struct-no-fields-4.rs:15     let _end_of_tuple = (3, Foo { });
