plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
....................i................................................................... 3960/3988
............................
failures:

---- src/cell/lazy.rs - cell::lazy::LazyCell<T,F>::into_inner (line 65) stdout ----
error[E0282]: type annotations needed for `LazyCell<T, [closure@src/cell/lazy.rs:10:26: 10:28]>`
   |
   |
10 | let lazy = LazyCell::new(|| hello.to_uppercase());
   |
   |
help: consider giving `lazy` an explicit type, where the type for type parameter `T` is specified
   |
10 | let lazy: LazyCell<T, [closure@src/cell/lazy.rs:10:26: 10:28]> = LazyCell::new(|| hello.to_uppercase());

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
For more information about this error, try `rustc --explain E0282`.
Couldn't compile the test.

failures:
    src/cell/lazy.rs - cell::lazy::LazyCell<T,F>::into_inner (line 65)

test result: FAILED. 3939 passed; 1 failed; 48 ignored; 0 measured; 0 filtered out; finished in 44.00s

error: doctest failed, to rerun pass `-p core --doc`
