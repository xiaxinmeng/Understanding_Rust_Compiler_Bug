rust
let foo: Box<T> = ...;
let bar = <Pin<Box<_>>>::from(foo);
let bar: Pin<Box<_>> = foo.into();
