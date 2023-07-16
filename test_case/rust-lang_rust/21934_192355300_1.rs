
hello.rs:5:13: 5:26 error: `Foo` is private, and cannot be reexported [E0364]
hello.rs:5     pub use A::Foo as Bar;
                       ^~~~~~~~~~~~~
hello.rs:5:13: 5:26 help: run `rustc --explain E0364` to see a detailed explanation
hello.rs:5:13: 5:26 note: consider marking `Foo` as `pub` in the imported module
hello.rs:5     pub use A::Foo as Bar;
                       ^~~~~~~~~~~~~
