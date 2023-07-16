
export PATH=/pkg/rustc-1.52.1-0/bin:$PATH LD_LIBRARY_PATH=/pkg/rustc-1.52.1-0/lib:$LD_LIBRARY_PATH RUSTFLAGS="$RUSTFLAGS -C link-args=-lffi"  &&
python3 ./x.py build --exclude src/tools/miri
[…]
   Compiling toml v0.5.7
error[E0277]: `[&str; 2]` is not an iterator
    --> src/bootstrap/compile.rs:1140:27
     |
1140 |             for flavor in ["ld", "ld64"] {
     |                           ^^^^^^^^^^^^^^ borrow the array with `&` or call `.iter()` on it to iterate over it
     |
     = help: the trait `Iterator` is not implemented for `[&str; 2]`
     = note: arrays are not iterators, but slices like the following are: `&[1, 2, 3]`
     = note: required because of the requirements on the impl of `IntoIterator` for `[&str; 2]`
     = note: required by `into_iter`

error[E0277]: `[&str; 2]` is not an iterator
   --> src/bootstrap/dist.rs:415:31
    |
415 |                 for flavor in ["ld", "ld64"] {
    |                               ^^^^^^^^^^^^^^ borrow the array with `&` or call `.iter()` on it to iterate over it
    |
    = help: the trait `Iterator` is not implemented for `[&str; 2]`
    = note: arrays are not iterators, but slices like the following are: `&[1, 2, 3]`
    = note: required because of the requirements on the impl of `IntoIterator` for `[&str; 2]`
    = note: required by `into_iter`

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
error: could not compile `bootstrap`
