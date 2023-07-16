rust
struct Bar<T: Clone>(T);

type Foo<T> = Bar<T>; // (*)
