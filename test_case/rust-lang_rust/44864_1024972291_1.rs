`
warning: use of deprecated macro `try`: use the `?` operator instead
  --> src/main.rs:4:1
   |
4  | / bitflags! {
5  | |     #[derive(Default)]
6  | |     pub struct Flags: u32 {
7  | |         const FLAG = 0x1;
8  | |         const MORE_FLAGS = 0x2;
9  | |     }
10 | | }
   | |_^
   |
   = note: `#[warn(deprecated)]` on by default
   = note: this warning originates in the macro `__impl_bitflags` (in Nightly builds, run with -Z macro-backtrace for more info)

warning: use of deprecated macro `try`: use the `?` operator instead
  --> src/main.rs:4:1
   |
4  | / bitflags! {
5  | |     #[derive(Default)]
6  | |     pub struct Flags: u32 {
7  | |         const FLAG = 0x1;
8  | |         const MORE_FLAGS = 0x2;
9  | |     }
10 | | }
   | |_^
   |
   = note: this warning originates in the macro `__impl_bitflags` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: the `?` operator can only be used on `Result`s, not `Option`s, in a function that returns `Result`
   --> src/main.rs:17:29
    |
16  | / fn pointless_fn(flags: Flags) -> Result<Flags, ()> {
17  | |     its_a_me_bitflags(flags)?;
    | |                             ^ use `.ok_or(...)?` to provide an error compatible with `Result<Flags, ()>`
18  | |     Ok(flags)
19  | | }
    | |_- this function returns a `Result`
    |
    = help: the trait `FromResidual<Option<Infallible>>` is not implemented for `Result<Flags, ()>`
note: required by `from_residual`

error[E0277]: `Flags` doesn't implement `std::fmt::Display`
  --> src/main.rs:23:33
   |
23 |         Ok(f) => println!("{}", f),
   |                                 ^ `Flags` cannot be formatted with the default formatter
   |
   = help: the trait `std::fmt::Display` is not implemented for `Flags`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
   = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
