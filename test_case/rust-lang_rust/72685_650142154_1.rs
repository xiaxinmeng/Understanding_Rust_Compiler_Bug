
error: internal compiler error: bad_placeholder_type
 --> src/main.rs:7:27
  |
7 | const FN_PTR: fn(_, _) -> _ = my_fn as fn(_, _) -> _;
  |                           ^
  |
  = note: delayed at /rustc/50fc24d8a172a853b5dfe40702d6550e3b8562ba/src/librustc_session/session.rs:436:27

error: internal compiler error: bad_placeholder_type
 --> src/main.rs:7:18
  |
7 | const FN_PTR: fn(_, _) -> _ = my_fn as fn(_, _) -> _;
  |                  ^
  |
  = note: delayed at /rustc/50fc24d8a172a853b5dfe40702d6550e3b8562ba/src/librustc_session/session.rs:436:27

error: internal compiler error: bad_placeholder_type
 --> src/main.rs:7:21
  |
7 | const FN_PTR: fn(_, _) -> _ = my_fn as fn(_, _) -> _;
  |                     ^
  |
  = note: delayed at /rustc/50fc24d8a172a853b5dfe40702d6550e3b8562ba/src/librustc_session/session.rs:436:27

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at /rustc/50fc24d8a172a853b5dfe40702d6550e3b8562ba/src/librustc_session/session.rs:436:27

error: internal compiler error: cat_expr Errd
 --> src/main.rs:7:31
  |
7 | const FN_PTR: fn(_, _) -> _ = my_fn as fn(_, _) -> _;
  |                               ^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: delayed at /rustc/50fc24d8a172a853b5dfe40702d6550e3b8562ba/src/librustc_session/session.rs:436:27

error: internal compiler error: cat_expr Errd
 --> src/main.rs:1:11
  |
1 |   fn main() {
  |  ___________^
2 | |     println!("Hello, world!");
3 | |     drop(FN_PTR);
4 | |     drop(my_fn);
5 | | }
  | |_^
  |
  = note: delayed at /rustc/50fc24d8a172a853b5dfe40702d6550e3b8562ba/src/librustc_session/session.rs:436:27

error: internal compiler error: cat_expr Errd
 --> src/main.rs:3:5
  |
3 |     drop(FN_PTR);
  |     ^^^^
  |
  = note: delayed at /rustc/50fc24d8a172a853b5dfe40702d6550e3b8562ba/src/librustc_session/session.rs:436:27

error: internal compiler error: cat_expr Errd
 --> src/main.rs:3:10
  |
3 |     drop(FN_PTR);
  |          ^^^^^^
  |
  = note: delayed at /rustc/50fc24d8a172a853b5dfe40702d6550e3b8562ba/src/librustc_session/session.rs:436:27

error: internal compiler error: PromoteTemps: MIR had errors
 --> src/main.rs:1:1
  |
1 | / fn main() {
2 | |     println!("Hello, world!");
3 | |     drop(FN_PTR);
4 | |     drop(my_fn);
5 | | }
  | |_^
  |
  = note: delayed at /rustc/50fc24d8a172a853b5dfe40702d6550e3b8562ba/src/librustc_session/session.rs:436:27

error: internal compiler error: broken MIR in DefId(0:3 ~ test[e008]::main[0]) ("return type"): bad type [type error]
 --> src/main.rs:1:1
  |
1 | / fn main() {
2 | |     println!("Hello, world!");
3 | |     drop(FN_PTR);
4 | |     drop(my_fn);
5 | | }
  | |_^
  |
  = note: delayed at src/librustc_mir/borrow_check/type_check/mod.rs:258:27

error: internal compiler error: broken MIR in DefId(0:3 ~ test[e008]::main[0]) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: src/main.rs:1:1: 5:2 (#0), scope: scope[0] } }): bad type [type error]
 --> src/main.rs:1:1
  |
1 | / fn main() {
2 | |     println!("Hello, world!");
3 | |     drop(FN_PTR);
4 | |     drop(my_fn);
5 | | }
  | |_^
  |
  = note: delayed at src/librustc_mir/borrow_check/type_check/mod.rs:258:27

error: internal compiler error: mir_const_qualif: MIR had errors
 --> src/main.rs:7:1
  |
7 | const FN_PTR: fn(_, _) -> _ = my_fn as fn(_, _) -> _;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: delayed at /rustc/50fc24d8a172a853b5dfe40702d6550e3b8562ba/src/librustc_session/session.rs:436:27

error: internal compiler error: PromoteTemps: MIR had errors
 --> src/main.rs:7:1
  |
7 | const FN_PTR: fn(_, _) -> _ = my_fn as fn(_, _) -> _;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: delayed at /rustc/50fc24d8a172a853b5dfe40702d6550e3b8562ba/src/librustc_session/session.rs:436:27

error: internal compiler error: broken MIR in DefId(0:4 ~ test[e008]::FN_PTR[0]) ("return type"): bad type [type error]
 --> src/main.rs:7:1
  |
7 | const FN_PTR: fn(_, _) -> _ = my_fn as fn(_, _) -> _;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: delayed at src/librustc_mir/borrow_check/type_check/mod.rs:258:27

error: internal compiler error: broken MIR in DefId(0:4 ~ test[e008]::FN_PTR[0]) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: src/main.rs:7:1: 7:54 (#0), scope: scope[0] } }): bad type [type error]
 --> src/main.rs:7:1
  |
7 | const FN_PTR: fn(_, _) -> _ = my_fn as fn(_, _) -> _;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: delayed at src/librustc_mir/borrow_check/type_check/mod.rs:258:27

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:367:17
stack backtrace:
   0:        0x10834cd4e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h13e6f26430148ff2
   1:        0x10838632c - core::fmt::write::h45a9bd04db15c24f
   2:        0x10833e567 - std::io::Write::write_fmt::h8ae61696f13218d5
   3:        0x108351885 - std::panicking::default_hook::{{closure}}::hc98fe2390b6284e5
   4:        0x1083515c2 - std::panicking::default_hook::h9a63cea866a4c14a
   5:        0x10139c988 - rustc_driver::report_ice::h6811b1f05e3fd746
   6:        0x108351ed5 - std::panicking::rust_panic_with_hook::h4d446ca45c8e1faa
   7:        0x105822abd - std::panicking::begin_panic::hba65cd1eb2ce6d69
   8:        0x105429e3f - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::h3a03d7831429fa48
   9:        0x1013d92ca - core::ptr::drop_in_place::he612e23a92ff1daf
  10:        0x1013dd728 - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::h86e165f15f416390
  11:        0x1013f3532 - core::ptr::drop_in_place::h6c089220cd3fa404
  12:        0x1013e92be - rustc_span::with_source_map::hcb5ea084b59b822f
  13:        0x101363d1a - rustc_interface::interface::run_compiler_in_existing_thread_pool::h4842a91ac763f9d6
  14:        0x101389479 - scoped_tls::ScopedKey<T>::set::haf496b5dd75c374f
  15:        0x1013a0cc2 - std::sys_common::backtrace::__rust_begin_short_backtrace::hac103380d443010e
  16:        0x10136acac - core::ops::function::FnOnce::call_once{{vtable.shim}}::hce68202ad15837a7
  17:        0x10835ff2d - std::sys::unix::thread::Thread::new::thread_start::h545d31fdf79b3d9e
  18:     0x7fff72d67109 - _ZL12preoptimized

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.46.0-nightly (50fc24d8a 2020-06-25) running on x86_64-apple-darwin

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `test`.

To learn more, run the command again with --verbose.
