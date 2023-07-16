plain
[00:00:56] configure: llvm.assertions      := True
[00:00:56] configure: build.locked-deps    := True
[00:00:56] configure: llvm.ccache          := sccache
[00:00:56] configure: build.openssl-static := True
[00:00:56] configure: build.configure-args := ['--enable-quiet-tests', '--enable-sccache', ' ...
[00:00:56] configure: writing `config.toml` in current directory
[00:00:56] configure: 
[00:00:56] configure: run `python /checkout/x.py --help`
[00:00:56] configure: 
---
[00:07:27]    Compiling rand v0.4.2
[00:07:27]    Compiling remove_dir_all v0.5.1
[00:07:28]    Compiling tempdir v0.3.7
[00:07:28]    Compiling rustdoc v0.0.0 (file:///checkout/src/librustdoc)
[00:07:29] error[E0658]: `dyn Trait` syntax is unstable (see issue #44662)
[00:07:29]     |
[00:07:29]     |
[00:07:29] 166 |     let emitter: Box<dyn Emitter + sync::Send> = match error_format {
[00:07:29]     |
[00:07:29]     |
[00:07:29]     = help: add #![feature(dyn_trait)] to the crate attributes to enable
[00:07:29] error: aborting due to previous error
[00:07:29] 
[00:07:29] For more information about this error, try `rustc --explain E0658`.
[00:07:29] error: Could not compile `rustdoc`.
[00:07:29] error: Could not compile `rustdoc`.
[00:07:29] 
[00:07:29] Caused by:
[00:07:29]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustdoc librustdoc/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,metadata -C debug-assertions=off -C overflow-checks=on -C metadata=53bc6cad8d025cf8 -C extra-filename=-53bc6cad8d025cf8 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/deps --target i686-pc-windows-gnu -C incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/incremental -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/release/deps --extern pulldown_cmark=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/deps/libpulldown_cmark-8f75380f3365e775.rmeta --extern tempdir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/i686-pc-windows-gnu/release/deps/libtempdir-4d964115e60069d9.rmeta -L native=/cargo/registry/src/github.com-1ecc6299db9ec823/winapi-i686-pc-windows-gnu-0.4.0/lib` (exit code: 101)
[00:07:29] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "i686-pc-windows-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "--message-format" "json"
[00:07:29] expected success, got: exit code: 101
[00:07:29] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:07:29] travis_fold:end:stage0-rustdoc

[00:07:29] travis_time:end:stage0-rustdoc:start=1524673242704764431,finish=1524673251062949500,duration=8358185069

