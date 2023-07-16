
$ cargo describe-future-incompatibilities -Z future-incompat-report --id IvWh
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
