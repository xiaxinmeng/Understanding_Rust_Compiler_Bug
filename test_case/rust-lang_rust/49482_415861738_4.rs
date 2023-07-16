
rm -fr incr *.so *.rlib; mkdir incr
$RUSTC binary_macros_impl.rs --crate-type=proc-macro -L.
$RUSTC binary_macros.rs --crate-type=rlib -L.
$RUSTC lib.rs --crate-type=rlib -L. -C incremental=$PWD/incr
