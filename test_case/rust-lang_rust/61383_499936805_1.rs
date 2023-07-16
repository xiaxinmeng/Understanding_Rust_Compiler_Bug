
error: internal compiler error: cat_expr Errd
  --> src/lib.rs:38:42
   |
38 |   fn default_impl<const N: usize>()->[();N]{
   |  __________________________________________^
39 | |     [(); N]
40 | | }
   | |_^

error: internal compiler error: cat_expr Errd
  --> src/lib.rs:39:5
   |
39 |     [(); N]
   |     ^^^^^^^

error: internal compiler error: QualifyAndPromoteConstants: MIR had errors
  --> src/lib.rs:38:1
   |
38 | / fn default_impl<const N: usize>()->[();N]{
39 | |     [(); N]
40 | | }
   | |_^

error: internal compiler error: broken MIR in DefId(0:12 ~ playground[92e4]::default_impl[0]) ("return type"): bad type [type error]
  --> src/lib.rs:38:1
   |
38 | / fn default_impl<const N: usize>()->[();N]{
39 | |     [(); N]
40 | | }
   | |_^

error: internal compiler error: broken MIR in DefId(0:12 ~ playground[92e4]::default_impl[0]) (LocalDecl { mutability: Mut, is_user_variable: None, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, name: None, source_info: SourceInfo { span: src/lib.rs:38:1: 40:2, scope: scope[0] }, visibility_scope: scope[0] }): bad type [type error]
  --> src/lib.rs:38:1
   |
38 | / fn default_impl<const N: usize>()->[();N]{
39 | |     [(); N]
40 | | }
   | |_^

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:357:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.37.0-nightly (5eeb567a2 2019-06-06) running on x86_64-unknown-linux-gnu

