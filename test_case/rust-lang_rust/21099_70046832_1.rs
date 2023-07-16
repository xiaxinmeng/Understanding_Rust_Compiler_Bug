
test.rs:2:28: 2:32 error: the trait `core::marker::Sized` is not implemented for the type `T`
test.rs:2 fn main() { let _: T = (|| loop {})(); }
                                     ^~~~
