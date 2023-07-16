
error[E0080]: constant evaluation error: the evaluated program panicked
   --> /home/r/src/rust/rustc/src/libstd/panicking.rs:525:9
    |
525 |         __rust_start_panic(obj as usize)
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked
    |
note: inside call to `std::panicking::rust_panic`
   --> /home/r/src/rust/rustc/src/libstd/panicking.rs:496:5
    |
496 |     rust_panic(payload)
    |     ^^^^^^^^^^^^^^^^^^^
note: inside call to `std::panicking::rust_panic_with_hook`
   --> /home/r/src/rust/rustc/src/libstd/panicking.rs:390:5
    |
390 | /     rust_panic_with_hook(
391 | |         &mut PanicPayload::new(msg),
392 | |         info.message(),
393 | |         &file_line_col);
    | |_______________________^
note: inside call to `std::panicking::continue_panic_fmt`
   --> /home/r/src/rust/rustc/src/libstd/panicking.rs:345:5
    |
345 |     continue_panic_fmt(&info)
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^
note: inside call to `std::rt::begin_panic_fmt`
   --> tests/compile-fail/panic.rs:4:5
    |
4   |     assert_eq!(5, 6);
    |     ^^^^^^^^^^^^^^^^^
note: inside call to `main`
   --> /home/r/src/rust/rustc/src/libstd/rt.rs:74:34
    |
74  |     lang_start_internal(&move || main().report(), argc, argv)
    |                                  ^^^^^^
note: inside call to `closure`
   --> /home/r/src/rust/rustc/src/libstd/rt.rs:59:75
    |
59  |             ::sys_common::backtrace::__rust_begin_short_backtrace(move || main())
    |                                                                           ^^^^^^
note: inside call to `closure`
   --> /home/r/src/rust/rustc/src/libstd/sys_common/backtrace.rs:136:5
    |
136 |     f()
    |     ^^^
note: inside call to `std::sys_common::backtrace::__rust_begin_short_backtrace::<[closure@DefId(1/1:1913 ~ std[78f0]::rt[0]::lang_start_internal[0]::{{closure}}[0]::{{closure}}[0]) 0:&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>`
   --> /home/r/src/rust/rustc/src/libstd/rt.rs:59:13
    |
59  |             ::sys_common::backtrace::__rust_begin_short_backtrace(move || main())
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
note: inside call to `closure`
   --> /home/r/src/rust/rustc/src/libstd/panicking.rs:310:40
    |
310 |             ptr::write(&mut (*data).r, f());
    |                                        ^^^
note: inside call to `std::panicking::try::do_call::<[closure@DefId(1/1:1912 ~ std[78f0]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>`
   --> /home/r/src/rust/rustc/src/libstd/panicking.rs:306:5
    |
306 | /     fn do_call<F: FnOnce() -> R, R>(data: *mut u8) {
307 | |         unsafe {
308 | |             let data = data as *mut Data<F, R>;
309 | |             let f = ptr::read(&mut (*data).f);
310 | |             ptr::write(&mut (*data).r, f());
311 | |         }
312 | |     }
    | |_____^
note: inside call to `std::panicking::try::<i32, [closure@DefId(1/1:1912 ~ std[78f0]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe]>`
   --> /home/r/src/rust/rustc/src/libstd/panic.rs:398:9
    |
398 |         panicking::try(f)
    |         ^^^^^^^^^^^^^^^^^
note: inside call to `std::panic::catch_unwind::<[closure@DefId(1/1:1912 ~ std[78f0]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>`
   --> /home/r/src/rust/rustc/src/libstd/rt.rs:58:25
    |
58  |           let exit_code = panic::catch_unwind(|| {
    |  _________________________^
59  | |             ::sys_common::backtrace::__rust_begin_short_backtrace(move || main())
60  | |         });
    | |__________^
note: inside call to `std::rt::lang_start_internal`
   --> /home/r/src/rust/rustc/src/libstd/rt.rs:74:5
    |
74  |     lang_start_internal(&move || main().report(), argc, argv)
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
