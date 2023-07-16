console
nixon@Ashtabula:/tmp/rust-107001$ cat code.rs
fn main() {
    let a = [1, 2, 3, 4, 5];
    let _x = a[a];

}
nixon@Ashtabula:/tmp/rust-107001$ rustc code.rs
error[E0277]: the type `[{integer}]` cannot be indexed by `[{integer}; 5]`
 --> code.rs:3:16
  |
3 |     let _x = a[a];
  |                ^ slice indices are of type `usize` or ranges of `usize`
  |
  = help: the trait `SliceIndex<[{integer}]>` is not implemented for `[{integer}; 5]`
  = note: required for `[{integer}]` to implement `Index<[{integer}; 5]>`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
nixon@Ashtabula:/tmp/rust-107001$ ls
code  code.rs
nixon@Ashtabula:/tmp/rust-107001$ vim code.rs
nixon@Ashtabula:/tmp/rust-107001$ cat code.rs
fn main() {
    let a = [1, 2, 3, 4, 5];
    let _x = a[9];

}
nixon@Ashtabula:/tmp/rust-107001$ rustc code.rs
error: this operation will panic at runtime
 --> code.rs:3:14
  |
3 |     let _x = a[9];
  |              ^^^^ index out of bounds: the length is 5 but the index is 9
  |
  = note: `#[deny(unconditional_panic)]` on by default

error: aborting due to previous error

nixon@Ashtabula:/tmp/rust-107001$ ls
code                          code.code.3969b7e3-cgu.0.rcgu.o  code.code.3969b7e3-cgu.2.rcgu.o  code.code.3969b7e3-cgu.4.rcgu.o
code.2w63xc29o8jhg8wq.rcgu.o  code.code.3969b7e3-cgu.1.rcgu.o  code.code.3969b7e3-cgu.3.rcgu.o  code.rs
