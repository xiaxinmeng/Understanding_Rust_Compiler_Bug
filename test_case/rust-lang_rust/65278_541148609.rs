plain
2019-10-11T14:43:40.2634303Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-11T14:43:40.2731696Z ##[command]git config gc.auto 0
2019-10-11T14:43:40.3314742Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-11T14:43:40.3384468Z ##[command]git config --get-all http.proxy
2019-10-11T14:43:40.3563558Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65278/merge:refs/remotes/pull/65278/merge
---
2019-10-11T16:25:24.9988699Z    Compiling mdbook-linkcheck v0.3.0
2019-10-11T16:25:42.1314524Z    Compiling rustbook v0.1.0 (/checkout/src/tools/rustbook)
2019-10-11T16:25:46.5641084Z     Finished release [optimized] target(s) in 8m 22s
2019-10-11T16:27:30.3196265Z Error: there are broken links
2019-10-11T16:27:30.3203099Z  Caused By: "***/tree/master/src/libsyntax/ext/tt" returned 404 Not Found
2019-10-11T16:27:30.3204018Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/fn.parse.html" returned 404 Not Found
2019-10-11T16:27:30.3204386Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_rules/" returned 404 Not Found
2019-10-11T16:27:30.3204746Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/syntax/ext/tt/macro_parser/" returned 404 Not Found
2019-10-11T16:27:30.3205090Z  Caused By: "https://rust-lang.github.io/chalk/doc/chalk_engine/forest/struct.Forest.html" returned 404 Not Found
---
2019-10-11T16:33:00.1324983Z    Compiling url v2.1.0
2019-10-11T16:33:10.5324214Z    Compiling toml v0.5.3
2019-10-11T16:33:39.4253486Z    Compiling cargo_metadata v0.8.0
2019-10-11T16:34:01.9197938Z    Compiling clippy_lints v0.0.212 (/checkout/src/tools/clippy/clippy_lints)
2019-10-11T16:40:03.1698130Z warning: use of deprecated attribute `plugin_registrar`: compiler plugins are deprecated. See ***/issues/29597
2019-10-11T16:40:03.1699938Z    |
2019-10-11T16:40:03.1700547Z 12 | #[plugin_registrar]
2019-10-11T16:40:03.1707033Z    | ^^^^^^^^^^^^^^^^^^^ help: remove this attribute
2019-10-11T16:40:03.1707675Z    |
---
2019-10-11T17:13:14.6777022Z normalized stderr:
2019-10-11T17:13:14.6777071Z error[E0080]: Miri evaluation error: the evaluated program panicked
2019-10-11T17:13:14.6777364Z   --> /checkout/src/libcore/panicking.rs:84:14
2019-10-11T17:13:14.6777411Z    |
2019-10-11T17:13:14.6777452Z 84 |     unsafe { panic_impl(&pi) }
2019-10-11T17:13:14.6777560Z    |
2019-10-11T17:13:14.6777608Z    = note: inside call to `core::panicking::panic_fmt` at /checkout/src/libcore/macros.rs:18:9
2019-10-11T17:13:14.6777692Z    = note: inside call to `std::result::unwrap_failed` at /checkout/src/libcore/result.rs:933:23
2019-10-11T17:13:14.6777692Z    = note: inside call to `std::result::unwrap_failed` at /checkout/src/libcore/result.rs:933:23
2019-10-11T17:13:14.6777748Z note: inside call to `std::result::Result::<std::fs::File, std::io::Error>::unwrap` at $DIR/fs.rs:11:20
2019-10-11T17:13:14.6778560Z    |
2019-10-11T17:13:14.6778605Z 11 |     let mut file = File::create(path).unwrap();
2019-10-11T17:13:14.6778653Z    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-10-11T17:13:14.6778721Z    = note: inside call to `main` at /checkout/src/libstd/rt.rs:61:34
2019-10-11T17:13:14.6778721Z    = note: inside call to `main` at /checkout/src/libstd/rt.rs:61:34
2019-10-11T17:13:14.6778773Z    = note: inside call to closure at /checkout/src/libstd/rt.rs:48:73
2019-10-11T17:13:14.6778826Z    = note: inside call to closure at /checkout/src/libstd/sys_common/backtrace.rs:129:5
2019-10-11T17:13:14.6779355Z    = note: inside call to `std::sys_common::backtrace::__rust_begin_short_backtrace::<[closure@DefId(1:6033 ~ std[1695]::rt[0]::lang_start_internal[0]::{{closure}}[0]::{{closure}}[0]) 0:&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/rt.rs:48:13
2019-10-11T17:13:14.6779434Z    = note: inside call to closure at /checkout/src/libstd/panicking.rs:288:40
2019-10-11T17:13:14.6780185Z    = note: inside call to `std::panicking::try::do_call::<[closure@DefId(1:6032 ~ std[1695]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/panicking.rs:284:5
2019-10-11T17:13:14.6780640Z    = note: inside call to `std::panicking::try::<i32, [closure@DefId(1:6032 ~ std[1695]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe]>` at /checkout/src/libstd/panic.rs:394:9
2019-10-11T17:13:14.6781096Z    = note: inside call to `std::panic::catch_unwind::<[closure@DefId(1:6032 ~ std[1695]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/rt.rs:47:25
2019-10-11T17:13:14.6781353Z    = note: inside call to `std::rt::lang_start::<()>`
2019-10-11T17:13:14.6781391Z 
2019-10-11T17:13:14.6781447Z error: aborting due to previous error
2019-10-11T17:13:14.6781474Z 
---
2019-10-11T17:13:14.6782062Z 
2019-10-11T17:13:14.6782102Z +error[E0080]: Miri evaluation error: the evaluated program panicked
2019-10-11T17:13:14.6782313Z +  --> /checkout/src/libcore/panicking.rs:84:14
2019-10-11T17:13:14.6782374Z +   |
2019-10-11T17:13:14.6782412Z +84 |     unsafe { panic_impl(&pi) }
2019-10-11T17:13:14.6782629Z +   |
2019-10-11T17:13:14.6782672Z +   = note: inside call to `core::panicking::panic_fmt` at /checkout/src/libcore/macros.rs:18:9
2019-10-11T17:13:14.6782728Z +   = note: inside call to `std::result::unwrap_failed` at /checkout/src/libcore/result.rs:933:23
2019-10-11T17:13:14.6782728Z +   = note: inside call to `std::result::unwrap_failed` at /checkout/src/libcore/result.rs:933:23
2019-10-11T17:13:14.6782791Z +note: inside call to `std::result::Result::<std::fs::File, std::io::Error>::unwrap` at $DIR/fs.rs:11:20
2019-10-11T17:13:14.6783060Z +   |
2019-10-11T17:13:14.6783116Z +11 |     let mut file = File::create(path).unwrap();
2019-10-11T17:13:14.6783158Z +   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-10-11T17:13:14.6783200Z +   = note: inside call to `main` at /checkout/src/libstd/rt.rs:61:34
2019-10-11T17:13:14.6783200Z +   = note: inside call to `main` at /checkout/src/libstd/rt.rs:61:34
2019-10-11T17:13:14.6783243Z +   = note: inside call to closure at /checkout/src/libstd/rt.rs:48:73
2019-10-11T17:13:14.6783314Z +   = note: inside call to closure at /checkout/src/libstd/sys_common/backtrace.rs:129:5
2019-10-11T17:13:14.6783743Z +   = note: inside call to `std::sys_common::backtrace::__rust_begin_short_backtrace::<[closure@DefId(1:6033 ~ std[1695]::rt[0]::lang_start_internal[0]::{{closure}}[0]::{{closure}}[0]) 0:&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/rt.rs:48:13
2019-10-11T17:13:14.6783835Z +   = note: inside call to closure at /checkout/src/libstd/panicking.rs:288:40
2019-10-11T17:13:14.6784234Z +   = note: inside call to `std::panicking::try::do_call::<[closure@DefId(1:6032 ~ std[1695]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/panicking.rs:284:5
2019-10-11T17:13:14.6784654Z +   = note: inside call to `std::panicking::try::<i32, [closure@DefId(1:6032 ~ std[1695]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe]>` at /checkout/src/libstd/panic.rs:394:9
2019-10-11T17:13:14.6785058Z +   = note: inside call to `std::panic::catch_unwind::<[closure@DefId(1:6032 ~ std[1695]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/rt.rs:47:25
2019-10-11T17:13:14.6785282Z +   = note: inside call to `std::rt::lang_start::<()>`
2019-10-11T17:13:14.6785319Z +
2019-10-11T17:13:14.6785372Z +error: aborting due to previous error
2019-10-11T17:13:14.6785407Z +
2019-10-11T17:13:14.6785407Z +
2019-10-11T17:13:14.6785663Z +For more information about this error, try `rustc --explain E0080`.
2019-10-11T17:13:14.6785723Z +
2019-10-11T17:13:14.6785745Z 
2019-10-11T17:13:14.6785782Z The actual stderr differed from the expected stderr.
2019-10-11T17:13:14.6785824Z Actual stderr saved to /tmp/compiletestU8fjVy/fs.stderr
2019-10-11T17:13:14.6785887Z To update references, run this command from build directory:
2019-10-11T17:13:14.6786120Z tests/run-pass/update-references.sh '/tmp/compiletestU8fjVy' 'fs.rs'
2019-10-11T17:13:14.6786197Z error: 1 errors occurred comparing output.
2019-10-11T17:13:14.6786253Z status: exit code: 1
2019-10-11T17:13:14.6786253Z status: exit code: 1
2019-10-11T17:13:14.6786767Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/fs.rs" "-L" "/tmp/compiletestU8fjVy" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletestU8fjVy/fs.stage-id" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-Zmiri-disable-isolation" "-L" "/tmp/compiletestU8fjVy/fs.stage-id.aux" "-A" "unused"
2019-10-11T17:13:14.6787048Z ------------------------------------------
2019-10-11T17:13:14.6787078Z 
2019-10-11T17:13:14.6787294Z ------------------------------------------
2019-10-11T17:13:14.6787334Z stderr:
2019-10-11T17:13:14.6787334Z stderr:
2019-10-11T17:13:14.6787618Z ------------------------------------------
2019-10-11T17:13:14.6796088Z {"message":"Miri evaluation error: the evaluated program panicked","code":{"code":"E0080","explanation":"\nThis error indicates that the compiler was unable to sensibly evaluate an\nconstant expression that had to be evaluated. Attempting to divide by 0\nor causing integer overflow are two ways to induce this error. For example:\n\n