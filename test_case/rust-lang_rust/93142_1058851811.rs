plain



The actual stderr differed from the expected stderr.
Actual stderr saved to /tmp/compiletest4q7oob/available-concurrency.stage-id.stderr
To only update this specific test, also pass `--test-args available-concurrency.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/run-pass/available-concurrency.rs" "-L" "/tmp/compiletest4q7oob" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/tmp/compiletest4q7oob/available-concurrency.stage-id" "-A" "unused" "--edition" "2018" "-Astable-features" "--sysroot" "/home/user/.cache/miri/HOST" "-L" "/tmp/compiletest4q7oob/available-concurrency.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"unsupported operation: `open` not available when isolation is enabled","code":null,"level":"error","spans":[{"file_name":"/checkout/library/std/src/sys/unix/fs.rs","byte_start":25298,"byte_end":25346,"line_start":802,"line_end":802,"column_start":36,"column_end":84,"is_primary":true,"text":[{"text":"        let fd = cvt_r(|| unsafe { open64(path.as_ptr(), flags, opts.mode as c_int) })?;","highlight_start":36,"highlight_end":84}],"label":"`open` not available when isolation is enabled","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"pass the flag `-Zmiri-disable-isolation` to disable isolation;","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"or pass `-Zmiri-isolation-error=warn to configure Miri to return an error code from isolated operations (if supported for that operation) and continue with a warning\n","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"inside closure at /checkout/library/std/src/sys/unix/fs.rs:802:36","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"inside `std::sys::unix::cvt_r::<i32, [closure@std::sys::unix::fs::File::open_c::{closure#0}]>` at /checkout/library/std/src/sys/unix/mod.rs:212:19","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"inside `std::sys::unix::fs::File::open_c` at /checkout/library/std/src/sys/unix/fs.rs:802:18","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"inside `std::sys::unix::fs::File::open` at /checkout/library/std/src/sys/unix/fs.rs:790:9","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"inside `std::fs::OpenOptions::_open` at /checkout/library/std/src/fs.rs:952:9","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"inside `std::fs::OpenOptions::open::<&std::path::Path>` at /checkout/library/std/src/fs.rs:948:9","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"inside `std::fs::File::open::<&str>` at /checkout/library/std/src/fs.rs:328:9","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"inside `std::sys::unix::thread::cgroup2_quota` at /checkout/library/std/src/sys/unix/thread.rs:391:9","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"inside `std::sys::unix::thread::available_parallelism` at /checkout/library/std/src/sys/unix/thread.rs:282:29","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"inside `std::thread::available_parallelism` at /checkout/library/std/src/thread/mod.rs:1562:5","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"inside `main` at tests/run-pass/available-concurrency.rs:2:16","code":null,"level":"note","spans":[{"file_name":"tests/run-pass/available-concurrency.rs","byte_start":27,"byte_end":63,"line_start":2,"line_end":2,"column_start":16,"column_end":52,"is_primary":true,"text":[{"text":"    assert_eq!(std::thread::available_parallelism().unwrap().get(), 1);","highlight_start":16,"highlight_end":52}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error: unsupported operation: `open` not available when isolation is enabled\n   --> /checkout/library/std/src/sys/unix/fs.rs:802:36\n    |\n802 |         let fd = cvt_r(|| unsafe { open64(path.as_ptr(), flags, opts.mode as c_int) })?;\n    |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `open` not available when isolation is enabled\n    |\n    = help: pass the flag `-Zmiri-disable-isolation` to disable isolation;\n    = help: or pass `-Zmiri-isolation-error=warn to configure Miri to return an error code from isolated operations (if supported for that operation) and continue with a warning\n            \n    = note: inside closure at /checkout/library/std/src/sys/unix/fs.rs:802:36\n    = note: inside `std::sys::unix::cvt_r::<i32, [closure@std::sys::unix::fs::File::open_c::{closure#0}]>` at /checkout/library/std/src/sys/unix/mod.rs:212:19\n    = note: inside `std::sys::unix::fs::File::open_c` at /checkout/library/std/src/sys/unix/fs.rs:802:18\n    = note: inside `std::sys::unix::fs::File::open` at /checkout/library/std/src/sys/unix/fs.rs:790:9\n    = note: inside `std::fs::OpenOptions::_open` at /checkout/library/std/src/fs.rs:952:9\n    = note: inside `std::fs::OpenOptions::open::<&std::path::Path>` at /checkout/library/std/src/fs.rs:948:9\n    = note: inside `std::fs::File::open::<&str>` at /checkout/library/std/src/fs.rs:328:9\n    = note: inside `std::sys::unix::thread::cgroup2_quota` at /checkout/library/std/src/sys/unix/thread.rs:391:9\n    = note: inside `std::sys::unix::thread::available_parallelism` at /checkout/library/std/src/sys/unix/thread.rs:282:29\n    = note: inside `std::thread::available_parallelism` at /checkout/library/std/src/thread/mod.rs:1562:5\nnote: inside `main` at tests/run-pass/available-concurrency.rs:2:16\n   --> tests/run-pass/available-concurrency.rs:2:16\n    |\n2   |     assert_eq!(std::thread::available_parallelism().unwrap().get(), 1);\n    |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n\n"}
{"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}

------------------------------------------

---

---- compile_test stdout ----
diff of stderr:

 error[E0601]: `main` function not found in crate `ice_6250`
-  --> $DIR/ice-6250.rs:4:1
+  --> $DIR/ice-6250.rs:16:2
-LL | / pub struct Cache {
-LL | |     data: Vec<i32>,
-LL | | }
-LL | |
-LL | |
-...  |
-LL | |     }
-LL | | }
-   | |_^ consider adding a `main` function to `$DIR/ice-6250.rs`
+LL | }
+   |  ^ consider adding a `main` function to `$DIR/ice-6250.rs`
 error[E0308]: mismatched types
   --> $DIR/ice-6250.rs:12:14
    |
    |
 LL |     for reference in vec![1, 2, 3] {
 ...
 ...
 LL |         Some(reference) = cache.data.get(key) {
    |              ^^^^^^^^^ expected integer, found `&i32`
 help: consider dereferencing the borrow
    |
    |
 LL |         Some(*reference) = cache.data.get(key) {
 
 error[E0308]: mismatched types
   --> $DIR/ice-6250.rs:12:9
    |
    |
 LL |         Some(reference) = cache.data.get(key) {
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `bool`, found `()`
 error: aborting due to 3 previous errors
 
 Some errors have detailed explanations: E0308, E0601.
 For more information about an error, try `rustc --explain E0308`.
---
To only update this specific test, also pass `--test-args crashes/ice-6250.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui/crashes/ice-6250.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/crashes/ice-6250.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-876c59a3e481cb18.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-e6439823b6d16f2c.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-c715357e8026769d.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f91723cecf6d8a5f.so" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-041fb6ac880e1ce0.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-6688bfb3a0699fc9.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-a8c1b2a71f554c3c.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-a436811527635382.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-12319133577eb155.so" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-c2a1ca34edf818c9.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-a2cb7849bbc8a2a2.rlib" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-8ccd5459decf8e02.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui/crashes/ice-6250.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"`main` function not found in crate `ice_6250`","code":{"code":"E0601","explanation":"No `main` function was found in a binary crate.\n\nTo fix this error, add a `main` function:\n\n