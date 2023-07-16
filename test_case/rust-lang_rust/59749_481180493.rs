plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:0523cfa8
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:06:29]    Compiling serde_derive v1.0.81
[00:06:30]    Compiling serde_json v1.0.33
[00:06:30]    Compiling toml v0.4.10
[00:06:39]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:06:46] error: function is never used: `invoke_rustdoc`
[00:06:46]     |
[00:06:46]     |
[00:06:46] 324 | / fn invoke_rustdoc(
[00:06:46] 325 | |     builder: &Builder<'_>,
[00:06:46] 327 | |     target: Interned<String>,
[00:06:46] ...   |
[00:06:46] ...   |
[00:06:46] 350 | |     builder.run(&mut cmd);
[00:06:46]     | |_^
[00:06:46]     |
[00:06:46] note: lint level defined here
[00:06:46]    --> src/bootstrap/lib.rs:107:9
[00:06:46]    --> src/bootstrap/lib.rs:107:9
[00:06:46]     |
[00:06:46] 107 | #![deny(warnings)]
[00:06:46]     |         ^^^^^^^^
[00:06:46]     = note: #[deny(dead_code)] implied by #[deny(warnings)]
[00:06:46] error: aborting due to previous error
[00:06:46] 
[00:06:46] error: Could not compile `bootstrap`.
[00:06:46] 
[00:06:46] 
[00:06:46] To learn more, run the command again with --verbose.
[00:06:46] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:06:46] Build completed unsuccessfully in 0:01:11
[00:06:46] make: *** [prepare] Error 1
[00:06:47] Command failed. Attempt 2/5:
[00:06:47]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:06:54] error: function is never used: `invoke_rustdoc`
[00:06:54]     |
[00:06:54]     |
[00:06:54] 324 | / fn invoke_rustdoc(
[00:06:54] 325 | |     builder: &Builder<'_>,
[00:06:54] 327 | |     target: Interned<String>,
[00:06:54] ...   |
[00:06:54] ...   |
[00:06:54] 350 | |     builder.run(&mut cmd);
[00:06:54]     | |_^
[00:06:54]     |
[00:06:54] note: lint level defined here
[00:06:54]    --> src/bootstrap/lib.rs:107:9
[00:06:54]    --> src/bootstrap/lib.rs:107:9
[00:06:54]     |
[00:06:54] 107 | #![deny(warnings)]
[00:06:54]     |         ^^^^^^^^
[00:06:54]     = note: #[deny(dead_code)] implied by #[deny(warnings)]
[00:06:55] error: aborting due to previous error
[00:06:55] 
[00:06:55] error: Could not compile `bootstrap`.
[00:06:55] 
[00:06:55] 
[00:06:55] To learn more, run the command again with --verbose.
[00:06:55] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:06:55] Build completed unsuccessfully in 0:00:07
[00:06:55] make: *** [prepare] Error 1
[00:06:57] Command failed. Attempt 3/5:
[00:06:57]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:07:04] error: function is never used: `invoke_rustdoc`
[00:07:04]     |
[00:07:04]     |
[00:07:04] 324 | / fn invoke_rustdoc(
[00:07:04] 325 | |     builder: &Builder<'_>,
[00:07:04] 327 | |     target: Interned<String>,
[00:07:04] ...   |
[00:07:04] ...   |
[00:07:04] 350 | |     builder.run(&mut cmd);
[00:07:04]     | |_^
[00:07:04]     |
[00:07:04] note: lint level defined here
[00:07:04]    --> src/bootstrap/lib.rs:107:9
[00:07:04]    --> src/bootstrap/lib.rs:107:9
[00:07:04]     |
[00:07:04] 107 | #![deny(warnings)]
[00:07:04]     |         ^^^^^^^^
[00:07:04]     = note: #[deny(dead_code)] implied by #[deny(warnings)]
[00:07:04] error: aborting due to previous error
[00:07:04] 
[00:07:04] error: Could not compile `bootstrap`.
[00:07:04] 
[00:07:04] 
[00:07:04] To learn more, run the command again with --verbose.
[00:07:04] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:07:04] Build completed unsuccessfully in 0:00:07
[00:07:04] make: *** [prepare] Error 1
[00:07:07] Command failed. Attempt 4/5:
[00:07:08]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:07:15] error: function is never used: `invoke_rustdoc`
[00:07:15]     |
[00:07:15]     |
[00:07:15] 324 | / fn invoke_rustdoc(
[00:07:15] 325 | |     builder: &Builder<'_>,
[00:07:15] 327 | |     target: Interned<String>,
[00:07:15] ...   |
[00:07:15] ...   |
[00:07:15] 350 | |     builder.run(&mut cmd);
[00:07:15]     | |_^
[00:07:15]     |
[00:07:15] note: lint level defined here
[00:07:15]    --> src/bootstrap/lib.rs:107:9
[00:07:15]    --> src/bootstrap/lib.rs:107:9
[00:07:15]     |
[00:07:15] 107 | #![deny(warnings)]
[00:07:15]     |         ^^^^^^^^
[00:07:15]     = note: #[deny(dead_code)] implied by #[deny(warnings)]
[00:07:15] error: aborting due to previous error
[00:07:15] 
[00:07:15] error: Could not compile `bootstrap`.
[00:07:15] 
[00:07:15] 
[00:07:15] To learn more, run the command again with --verbose.
[00:07:15] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:07:15] Build completed unsuccessfully in 0:00:07
[00:07:15] make: *** [prepare] Error 1
[00:07:19] Command failed. Attempt 5/5:
[00:07:19]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:07:26] error: function is never used: `invoke_rustdoc`
[00:07:26]     |
[00:07:26]     |
[00:07:26] 324 | / fn invoke_rustdoc(
[00:07:26] 325 | |     builder: &Builder<'_>,
[00:07:26] 327 | |     target: Interned<String>,
[00:07:26] ...   |
[00:07:26] ...   |
[00:07:26] 350 | |     builder.run(&mut cmd);
[00:07:26]     | |_^
[00:07:26]     |
[00:07:26] note: lint level defined here
[00:07:26]    --> src/bootstrap/lib.rs:107:9
[00:07:26]    --> src/bootstrap/lib.rs:107:9
[00:07:26]     |
[00:07:26] 107 | #![deny(warnings)]
[00:07:26]     |         ^^^^^^^^
[00:07:26]     = note: #[deny(dead_code)] implied by #[deny(warnings)]
[00:07:26] error: aborting due to previous error
[00:07:26] 
[00:07:26] error: Could not compile `bootstrap`.
[00:07:26] 
---
travis_time:end:02ef45a0:start=1554802968852395778,finish=1554802968869669400,duration=17273622
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1ec83cdc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02676d8c
travis_time:start:02676d8c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00fd891e
$ dmesg | grep -i kill
