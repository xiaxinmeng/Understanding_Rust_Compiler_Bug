
     Running `rustc --crate-name ice --edition=2018 src\lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 -C metadata=1c3553409c4d9998 -C extra-filename=-1c3553409c4d9998 --out-dir "target\debug\deps" -C "incremental=target\debug\incremental" -L "dependency=target\debug\deps"`
error: internal compiler error: compiler\rustc_middle\src\ich\impls_ty.rs:94:17: StableHasher: unexpected region '_#0r

thread 'rustc' panicked at 'Box<dyn Any>', compiler\rustc_errors\src\lib.rs:1034:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.0-nightly (08095fc1f 2021-07-26) running on x86_64-pc-windows-msvc

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [check_impl_item_well_formed] checking that `<impl at src\lib.rs:3:1: 10:2>::IntoIter` is well-formed
#1 [analysis] running analysis passes on this crate
end of query stack
error[E0277]: `&T` is not an iterator
   --> src\lib.rs:7:27
    |
7   |     fn into_iter(self) -> Self::IntoIter {
    |                           ^^^^^^^^^^^^^^ `&T` is not an iterator
    |
   ::: .rustup\toolchains\nightly-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\adapters\flatten.rs:150:38
    |
150 | pub struct Flatten<I: Iterator<Item: IntoIterator>> {
    |                                      ------------ required by this bound in `Flatten`
    |
    = help: the trait `Iterator` is not implemented for `&T`
    = note: required because of the requirements on the impl of `IntoIterator` for `&T`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `ice` due to previous error

Caused by:
  process didn't exit successfully: `rustc --crate-name ice --edition=2018 src\lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata -C embed-bitcode=no -C debuginfo=2 -C metadata=1c3553409c4d9998 -C extra-filename=-1c3553409c4d9998 --out-dir "target\debug\deps" -C "incremental=target\debug\incremental" -L "dependency=target\debug\deps"` (exit code: 101)
