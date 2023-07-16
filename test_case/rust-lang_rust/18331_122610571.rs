
> rustc --version
rustc 1.3.0-nightly (e4e93196e 2015-07-14)
> rustc --cfg phantom foo.rs
foo.rs:9:13: 9:15 error: parameter `'a` is never used [E0392]
foo.rs:9 struct Node<'a>;
                     ^~
foo.rs:9:13: 9:15 help: consider removing `'a` or using a marker such as `core::marker::PhantomData`
