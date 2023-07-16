
~ $ rustc --version
rustc 1.62.0
~ $ rustc --version --verbose
rustc 1.62.0
binary: rustc
commit-hash: unknown
commit-date: unknown
host: riscv64-alpine-linux-musl
release: 1.62.0
LLVM version: 14.0.6
~ $ rustc test1.rs 
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
 --> test1.rs:1:38
  |
1 | fn main() { let mut f = Result::Ok(5)?; }
  | -------------------------------------^---
  | |                                    |
  | |                                    cannot use the `?` operator in a function that returns `()`
  | this function should return `Result` or `Option` to accept `?`
  |
  = help: the trait `FromResidual<Result<Infallible, _>>` is not implemented for `()`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
~ $ rustc test1.rs 
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
 --> test1.rs:1:38
  |
1 | fn main() { let mut f = Result::Ok(5)?; }
  | -------------------------------------^---
  | |                                    |
  | |                                    cannot use the `?` operator in a function that returns `()`
  | this function should return `Result` or `Option` to accept `?`
  |
  = help: the trait `FromResidual<Result<Infallible, _>>` is not implemented for `()`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
~ $ rustc test2.rs 
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
 --> test2.rs:1:29
  |
1 | fn main() { let mut f = None?; }
  | ----------------------------^---
  | |                           |
  | |                           cannot use the `?` operator in a function that returns `()`
  | this function should return `Result` or `Option` to accept `?`
  |
  = help: the trait `FromResidual<Option<Infallible>>` is not implemented for `()`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
~ $ uname -a
Linux ncopa-edge-riscv64 5.15.11-0-lts #1-Alpine SMP Thu, 23 Dec 2021 08:03:24 +0000 riscv64 GNU/Linux
~ $
