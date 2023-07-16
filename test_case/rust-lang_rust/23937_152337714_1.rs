
hello.rs:9:9: 9:19 error: `Foo` is private, and cannot be reexported [E0364]
hello.rs:9 pub use Foo as Bar;
                   ^~~~~~~~~~
hello.rs:9:9: 9:19 help: run `rustc --explain E0364` to see a detailed explanation
hello.rs:9:9: 9:19 note: Consider marking `Foo` as `pub` in the imported module
hello.rs:9 pub use Foo as Bar;
                   ^~~~~~~~~~
