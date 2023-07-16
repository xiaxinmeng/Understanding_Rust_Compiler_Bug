plain
[00:01:02] configure: llvm.assertions      := True
[00:01:02] configure: build.locked-deps    := True
[00:01:02] configure: llvm.ccache          := sccache
[00:01:02] configure: build.openssl-static := True
[00:01:02] configure: build.configure-args := ['--enable-quiet-tests', '--enable-sccache', ' ...
[00:01:02] configure: writing `config.toml` in current directory
[00:01:02] configure: 
[00:01:02] configure: run `python /checkout/x.py --help`
[00:01:02] configure: 
---
[00:04:06]    Compiling term v0.0.0 (file:///checkout/src/libterm)
[00:04:06]    Compiling getopts v0.2.17
[00:04:06] error[E0463]: can't find crate for `std`
[00:04:06]   |
[00:04:06]   = note: the `i686-pc-windows-gnu` target may not be installed
[00:04:06] error: aborting due to previous error
[00:04:06] 
[00:04:06] For more information about this error, try `rustc --explain E0463`.
[00:04:06] For more information about this error, try `rustc --explain E0463`.
[00:04:06] error: Could not compile `getopts`.
[00:04:06] Caused by:
[00:04:06] Caused by:
[00:04:06]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name getopts /cargo/registry/src/github.com-1ecc6299db9ec823/getopts-0.2.17/src/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,metadata -C debug-assertions=off -C overflow-checks=on -C metadata=4e08df4e353c985f -C extra-filename=-4e08df4e353c985f --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-test/i686-pc-windows-gnu/release/deps --target i686-pc-windows-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-test/i686-pc-windows-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-test/release/deps --cap-lints allow` (exit code: 101)
[00:04:06] error[E0463]: can't find crate for `std`
[00:04:06]   |
[00:04:06]   |
[00:04:06]   = note: the `i686-pc-windows-gnu` target may not be installed
[00:04:06] error: aborting due to previous error
[00:04:06] 
[00:04:06] For more information about this error, try `rustc --explain E0463`.
[00:04:06] For more information about this error, try `rustc --explain E0463`.
[00:04:06] error: Could not compile `term`.
[00:04:06] Caused by:
[00:04:06] Caused by:
[00:04:06]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name term libterm/lib.rs --color always --error-format json --crate-type dylib --crate-type rlib --emit=dep-info,metadata -C prefer-dynamic -C debug-assertions=off -C overflow-checks=on -C metadata=ef408614dfbdda3b -C extra-filename=-ef408614dfbdda3b --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-test/i686-pc-windows-gnu/release/deps --target i686-pc-windows-gnu -C incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-test/i686-pc-windows-gnu/release/incremental -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-test/i686-pc-windows-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-test/release/deps` (exit code: 101)
[00:04:06] command did not execute successfully: "/checkout/obj/build/base/x86_64-unknown-linux-gnu/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json"
[00:04:06] expected success, got: exit code: 101
[00:04:06] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1162:9
[00:04:06] travis_fold:end:stage0-test

[00:04:06] travis_time:end:stage0-test:start=1524177679072080950,finish=1524177679789487852,duration=717406902

