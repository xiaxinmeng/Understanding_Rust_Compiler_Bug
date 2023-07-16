 rust
test.rs:2:25: 4:2 note: consider using an explicit lifetime parameter as shown: fn foo<'a>(x: &'a Foo) -> &'a int
test.rs:2 fn foo(x: &Foo) -> &int {
