rust
struct Foo<T>(ManuallyDrop<T>);

let foo = Box::new(Foo(ManuallyDrop::new(5i32))) as Box<Foo<::std::any::Any>;
