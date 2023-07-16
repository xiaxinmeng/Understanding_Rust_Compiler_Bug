
error: internal compiler error[E0308]: match arms have incompatible types
 --> src/main.rs:6:14
  |
3 | /     match 0_i32 {
4 | |         0 => true,
  | |              ---- this is found to be of type `bool`
5 | |         1 => true,
  | |              ---- this is found to be of type `bool`
6 | |         _ => i = (),
  | |              ^^^^^^ expected `bool`, found `()`
7 | |     };
  | |_____- `match` arms have incompatible types

error: internal compiler error: cat_expr Errd
 --> src/main.rs:1:11
  |
1 |   fn main() {
  |  ___________^
2 | |     let i;
3 | |     match 0_i32 {
4 | |         0 => true,
... |
7 | |     };
8 | | }
  | |_^

error: internal compiler error: cat_expr Errd
 --> src/main.rs:3:5
  |
3 | /     match 0_i32 {
4 | |         0 => true,
5 | |         1 => true,
6 | |         _ => i = (),
7 | |     };
  | |_____^

error: internal compiler error: PromoteTemps: MIR had errors
 --> src/main.rs:1:1
  |
1 | / fn main() {
2 | |     let i;
3 | |     match 0_i32 {
4 | |         0 => true,
... |
7 | |     };
8 | | }
  | |_^

error: internal compiler error: broken MIR in DefId(0:3 ~ foo[a7ed]::main[0]) ("return type"): bad type [type error]
 --> src/main.rs:1:1
  |
1 | / fn main() {
2 | |     let i;
3 | |     match 0_i32 {
4 | |         0 => true,
... |
7 | |     };
8 | | }
  | |_^

error: internal compiler error: broken MIR in DefId(0:3 ~ foo[a7ed]::main[0]) (LocalDecl { mutability: Mut, local_info: Other, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, source_info: SourceInfo { span: src/main.rs:1:1: 8:2, scope: scope[0] } }): bad type [type error]
 --> src/main.rs:1:1
  |
1 | / fn main() {
2 | |     let i;
3 | |     match 0_i32 {
4 | |         0 => true,
... |
7 | |     };
8 | | }
  | |_^

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:347:17
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.41.0-nightly (27d6f55f4 2019-12-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden
