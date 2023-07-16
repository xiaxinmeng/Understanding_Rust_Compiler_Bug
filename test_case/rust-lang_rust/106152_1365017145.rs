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
---- src/cell/lazy.rs - cell::lazy::LazyCell<T,F>::into_inner (line 65) stdout ----
error[E0282]: type annotations needed
  --> src/cell/lazy.rs:74:31
   |
12 | assert_eq!(lazy.into_inner(), None);
   |                               ^^^^ cannot infer type of the type parameter `T` declared on the enum `Option`
help: consider specifying the generic argument
   |
   |
12 | assert_eq!(lazy.into_inner(), None::<T>);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
For more information about this error, try `rustc --explain E0282`.
Couldn't compile the test.

failures:
    src/cell/lazy.rs - cell::lazy::LazyCell<T,F>::into_inner (line 65)

test result: FAILED. 3939 passed; 1 failed; 48 ignored; 0 measured; 0 filtered out; finished in 57.10s

error: doctest failed, to rerun pass `-p core --doc`
