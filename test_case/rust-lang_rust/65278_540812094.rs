plain
2019-10-10T19:22:33.6685135Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-10T19:22:33.6739966Z ##[command]git config gc.auto 0
2019-10-10T19:22:33.6742566Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-10T19:22:33.6744048Z ##[command]git config --get-all http.proxy
2019-10-10T19:22:33.6746257Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65278/merge:refs/remotes/pull/65278/merge
---
2019-10-10T20:55:50.8701187Z    Compiling mdbook-linkcheck v0.3.0
2019-10-10T20:56:02.9229265Z    Compiling rustbook v0.1.0 (/checkout/src/tools/rustbook)
2019-10-10T20:56:06.9413260Z     Finished release [optimized] target(s) in 7m 41s
2019-10-10T20:58:00.7334762Z Error: there are broken links
2019-10-10T20:58:00.7337025Z  Caused By: "***/tree/master/src/libsyntax/ext/tt" returned 404 Not Found
2019-10-10T20:58:00.7340549Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/fn.parse.html" returned 404 Not Found
2019-10-10T20:58:00.7340813Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_rules/" returned 404 Not Found
2019-10-10T20:58:00.7341591Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/" returned 404 Not Found
2019-10-10T20:58:00.7341851Z  Caused By: "https://rust-lang.github.io/chalk/doc/chalk_engine/forest/struct.Forest.html" returned 404 Not Found
---
2019-10-10T21:03:12.0876822Z    Compiling toml v0.5.3
2019-10-10T21:03:21.7708763Z    Compiling url v2.1.0
2019-10-10T21:03:49.3037223Z    Compiling cargo_metadata v0.8.0
2019-10-10T21:04:10.6754483Z    Compiling clippy_lints v0.0.212 (/checkout/src/tools/clippy/clippy_lints)
2019-10-10T21:09:44.5588505Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-10T21:09:44.5589140Z    |
2019-10-10T21:09:44.5589373Z 12 | #[plugin_registrar]
2019-10-10T21:09:44.5589979Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-10T21:09:44.5590164Z    |
---
2019-10-10T21:40:58.9132797Z normalized stderr:
2019-10-10T21:40:58.9133217Z error[E0080]: Miri evaluation error: the evaluated program panicked
2019-10-10T21:40:58.9133880Z   --> /checkout/src/libcore/panicking.rs:84:14
2019-10-10T21:40:58.9134029Z    |
2019-10-10T21:40:58.9134065Z 84 |     unsafe { panic_impl(&pi) }
2019-10-10T21:40:58.9134138Z    |
2019-10-10T21:40:58.9134195Z    = note: inside call to `core::panicking::panic_fmt` at /checkout/src/libcore/macros.rs:18:9
2019-10-10T21:40:58.9134238Z    = note: inside call to `std::result::unwrap_failed` at /checkout/src/libcore/result.rs:933:23
2019-10-10T21:40:58.9134238Z    = note: inside call to `std::result::unwrap_failed` at /checkout/src/libcore/result.rs:933:23
2019-10-10T21:40:58.9134281Z note: inside call to `std::result::Result::<std::fs::File, std::io::Error>::unwrap` at $DIR/file_manipulation.rs:11:20
2019-10-10T21:40:58.9134598Z    |
2019-10-10T21:40:58.9134634Z 11 |     let mut file = File::create(path).unwrap();
2019-10-10T21:40:58.9134697Z    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-10-10T21:40:58.9134735Z    = note: inside call to `main` at /checkout/src/libstd/rt.rs:61:34
2019-10-10T21:40:58.9134735Z    = note: inside call to `main` at /checkout/src/libstd/rt.rs:61:34
2019-10-10T21:40:58.9134772Z    = note: inside call to closure at /checkout/src/libstd/rt.rs:48:73
2019-10-10T21:40:58.9134836Z    = note: inside call to closure at /checkout/src/libstd/sys_common/backtrace.rs:129:5
2019-10-10T21:40:58.9135228Z    = note: inside call to `std::sys_common::backtrace::__rust_begin_short_backtrace::<[closure@DefId(1:6033 ~ std[1695]::rt[0]::lang_start_internal[0]::{{closure}}[0]::{{closure}}[0]) 0:&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/rt.rs:48:13
2019-10-10T21:40:58.9135304Z    = note: inside call to closure at /checkout/src/libstd/panicking.rs:288:40
2019-10-10T21:40:58.9135655Z    = note: inside call to `std::panicking::try::do_call::<[closure@DefId(1:6032 ~ std[1695]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/panicking.rs:284:5
2019-10-10T21:40:58.9136021Z    = note: inside call to `std::panicking::try::<i32, [closure@DefId(1:6032 ~ std[1695]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe]>` at /checkout/src/libstd/panic.rs:394:9
2019-10-10T21:40:58.9136987Z    = note: inside call to `std::panic::catch_unwind::<[closure@DefId(1:6032 ~ std[1695]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/rt.rs:47:25
2019-10-10T21:40:58.9137142Z    = note: inside call to `std::rt::lang_start::<()>`
2019-10-10T21:40:58.9137376Z 
2019-10-10T21:40:58.9137420Z error: aborting due to previous error
2019-10-10T21:40:58.9137466Z 
---
2019-10-10T21:40:58.9138053Z 
2019-10-10T21:40:58.9138122Z +error[E0080]: Miri evaluation error: the evaluated program panicked
2019-10-10T21:40:58.9138377Z +  --> /checkout/src/libcore/panicking.rs:84:14
2019-10-10T21:40:58.9138427Z +   |
2019-10-10T21:40:58.9138488Z +84 |     unsafe { panic_impl(&pi) }
2019-10-10T21:40:58.9138587Z +   |
2019-10-10T21:40:58.9138637Z +   = note: inside call to `core::panicking::panic_fmt` at /checkout/src/libcore/macros.rs:18:9
2019-10-10T21:40:58.9138721Z +   = note: inside call to `std::result::unwrap_failed` at /checkout/src/libcore/result.rs:933:23
2019-10-10T21:40:58.9138721Z +   = note: inside call to `std::result::unwrap_failed` at /checkout/src/libcore/result.rs:933:23
2019-10-10T21:40:58.9138883Z +note: inside call to `std::result::Result::<std::fs::File, std::io::Error>::unwrap` at $DIR/file_manipulation.rs:11:20
2019-10-10T21:40:58.9139271Z +   |
2019-10-10T21:40:58.9139320Z +11 |     let mut file = File::create(path).unwrap();
2019-10-10T21:40:58.9139373Z +   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-10-10T21:40:58.9139445Z +   = note: inside call to `main` at /checkout/src/libstd/rt.rs:61:34
2019-10-10T21:40:58.9139445Z +   = note: inside call to `main` at /checkout/src/libstd/rt.rs:61:34
2019-10-10T21:40:58.9139500Z +   = note: inside call to closure at /checkout/src/libstd/rt.rs:48:73
2019-10-10T21:40:58.9139557Z +   = note: inside call to closure at /checkout/src/libstd/sys_common/backtrace.rs:129:5
2019-10-10T21:40:58.9140100Z +   = note: inside call to `std::sys_common::backtrace::__rust_begin_short_backtrace::<[closure@DefId(1:6033 ~ std[1695]::rt[0]::lang_start_internal[0]::{{closure}}[0]::{{closure}}[0]) 0:&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/rt.rs:48:13
2019-10-10T21:40:58.9140348Z +   = note: inside call to closure at /checkout/src/libstd/panicking.rs:288:40
2019-10-10T21:40:58.9140899Z +   = note: inside call to `std::panicking::try::do_call::<[closure@DefId(1:6032 ~ std[1695]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/panicking.rs:284:5
2019-10-10T21:40:58.9141414Z +   = note: inside call to `std::panicking::try::<i32, [closure@DefId(1:6032 ~ std[1695]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe]>` at /checkout/src/libstd/panic.rs:394:9
2019-10-10T21:40:58.9141769Z +   = note: inside call to `std::panic::catch_unwind::<[closure@DefId(1:6032 ~ std[1695]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/rt.rs:47:25
2019-10-10T21:40:58.9141871Z +   = note: inside call to `std::rt::lang_start::<()>`
2019-10-10T21:40:58.9141920Z +
2019-10-10T21:40:58.9141952Z +error: aborting due to previous error
2019-10-10T21:40:58.9141983Z +
2019-10-10T21:40:58.9141983Z +
2019-10-10T21:40:58.9142190Z +For more information about this error, try `rustc --explain E0080`.
2019-10-10T21:40:58.9142226Z +
2019-10-10T21:40:58.9142246Z 
2019-10-10T21:40:58.9142279Z The actual stderr differed from the expected stderr.
2019-10-10T21:40:58.9142334Z Actual stderr saved to /tmp/compiletestUY58oE/file_manipulation.stderr
2019-10-10T21:40:58.9142371Z To update references, run this command from build directory:
2019-10-10T21:40:58.9142577Z tests/run-pass/update-references.sh '/tmp/compiletestUY58oE' 'file_manipulation.rs'
2019-10-10T21:40:58.9142733Z error: 1 errors occurred comparing output.
2019-10-10T21:40:58.9142766Z status: exit code: 1
2019-10-10T21:40:58.9142766Z status: exit code: 1
2019-10-10T21:40:58.9143285Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/file_manipulation.rs" "-L" "/tmp/compiletestUY58oE" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestUY58oE/file_manipulation.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-disable-isolation" "-L" "/tmp/compiletestUY58oE/file_manipulation.stage-id.aux" "-A" "unused"
2019-10-10T21:40:58.9143524Z ------------------------------------------
2019-10-10T21:40:58.9143567Z 
2019-10-10T21:40:58.9143738Z ------------------------------------------
2019-10-10T21:40:58.9143773Z stderr:
2019-10-10T21:40:58.9143773Z stderr:
2019-10-10T21:40:58.9143953Z ------------------------------------------
2019-10-10T21:40:58.9151362Z {"message":"Miri evaluation error: the evaluated program panicked","code":{"code":"E0080","explanation":"\nThis error indicates that the compiler was unable to sensibly evaluate an\nconstant expression that had to be evaluated. Attempting to divide by 0\nor causing integer overflow are two ways to induce this error. For example:\n\n