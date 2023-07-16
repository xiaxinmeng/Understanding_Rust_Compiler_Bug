
$ cargo check --future-incompat-report -Z unstable-options -Z future-incompat-report
    Updating crates.io index
  Downloaded lazy_static v0.1.16
  Downloaded colored v1.1.0
  Downloaded 2 crates (700.7 KB) in 1.35s
    Checking lazy_static v0.1.16
    Checking colored v1.1.0
    Checking futtest v0.1.0 (/home/lukas/dev/futtest)
    Finished dev [unoptimized + debuginfo] target(s) in 11.66s
warning: the following crates contain code that will be rejected by a future version of Rust: colored v1.1.0
The crate `colored v1.1.0` currently triggers the following future incompatibility lints:
> warning: this method call currently resolves to `<&[T; N] as IntoIterator>::into_iter` (due to autoref coercions), but that might change in the future when `IntoIterator` impls for arrays are added.
>   --> /home/lukas/.cargo/registry/src/github.com-1ecc6299db9ec823/colored-1.1.0/src/style.rs:71:40
>    |
> 71 |         let res : Vec<Styles> = STYLES.into_iter()
>    |                                        ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
>    |
>    = note: `#[allow(array_into_iter)]` on by default
>    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
>    = note: for more information, see issue #66145 <https://github.com/rust-lang/rust/issues/66145>
> 
note: this report can be shown with `cargo describe-future-incompatibilities -Z future-incompat-report --id Z0Tf`
