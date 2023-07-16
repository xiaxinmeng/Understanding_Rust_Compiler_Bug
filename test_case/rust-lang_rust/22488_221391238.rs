 text
foo.rs:1:10: 1:11 error: parameter `T` is never used [E0392]
foo.rs:1 struct U<T> {
                  ^
foo.rs:1:10: 1:11 help: run `rustc --explain E0392` to see a detailed explanation
foo.rs:1:10: 1:11 help: consider removing `T` or using a marker such as `core::marker::PhantomData`
