plain
travis_time:end:0331ef0f:start=1557722405009871566,finish=1557722492595699672,duration=87585828106
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:38:24]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:38:25] error: Could not compile `syntax_pos`.
[00:38:25] 
[00:38:25] Caused by:
[00:38:25]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --edition=2018 --crate-name syntax_pos src/libsyntax_pos/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=d46843837a41024c -C extra-filename=-d46843837a41024c --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-14b5fc1195912829.so --extern cfg_if=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libcfg_if-25f7973b49aaf9c0.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-3b12615f7748fb48.so --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps/librustc_macros-ad4581548d76fa48.so --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-fee9662c14477a45.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-65c4157205da4e40.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-65c4157205da4e40.rlib --extern unicode_width=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libunicode_width-ec3a485b818c54ba.rlib` (signal: 11, SIGSEGV: invalid memory reference)
[00:39:46] error: build failed
[00:39:46] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:39:46] expected success, got: exit code: 101
[00:39:46] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
---
travis_time:end:12096cec:start=1557724889925403017,finish=1557724889932235852,duration=6832835
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0711a934
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:start:crashlog
obj/cores/core.10397.!checkout!obj!build!x86_64-unknown-linux-gnu!stage1!bin!rustc
[New LWP 10398]
[New LWP 10397]
warning: Could not load shared library symbols for 14 libraries, e.g. /lib/x86_64-linux-gnu/libc.so.6.
Use the "info sharedlibrary" command to see the complete listing.
Do you need "set solib-search-path" or "set sysroot"?
ustc_interface..interface..Compiler$GT$::expansion::h3c4e5511d7b6c0fa () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/librustc_interface-d40570b20b31e1f6.so
#16 0x00007fd51837264b in rustc_interface::queries::_$LT$impl$u20$rustc_interface..interface..Compiler$GT$::lower_to_hir::h9dfdda6305d8b8b4 () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/librustc_interface-d40570b20b31e1f6.so
#17 0x00007fd518372cc9 in rustc_interface::queries::_$LT$impl$u20$rustc_interface..interface..Compiler$GT$::prepare_outputs::hf555e74849db2284 () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/../lib/librustc_interface-d40570b20b31e1f6.so
#18 0x00007fd518f52918 in rustc_interface::interface::run_compiler_in_existing_thread_pool::h8c6409a23c1fa7a7 () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-60a1cd996b8c2db5.so
#19 0x00007fd518fc4366 in std::thread::local::LocalKey$LT$T$GT$::with::ha46e068d86419b52 () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-60a1cd996b8c2db5.so
#20 0x00007fd518f69765 in scoped_tls::ScopedKey$LT$T$GT$::set::h0f66fae6d7ca732c () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-60a1cd996b8c2db5.so
#21 0x00007fd518fa15c2 in syntax::with_globals::h25baa84e87b093e6 () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/../lib/librustc_driver-60a1cd996b8c2db5.so
#22 0x00007fd518ede115 in std::sys_common::backtrace::__rust_begin_short_backtrace::h06fb4440f32b393c () from ./checkout/obj/build/x86_64-unknown-linux-gnu/stage
