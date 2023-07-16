
foo.rs:11:0: 15:1 error: duplicate definition of type `Foo`
foo.rs:11 impl<T: Freeze> Foo<T> {
foo.rs:12     fn from_const(value: T) -> Foo<T> {
foo.rs:13         Foo{value: value}
foo.rs:14     }
foo.rs:15 }
foo.rs:5:0: 9:1 note: first definition of type Foo here:
foo.rs:5 impl<T: Send> Foo<T> {
foo.rs:6     fn from_owned(value: T) -> Foo<T> {
foo.rs:7         Foo{value: value}
foo.rs:8     }
foo.rs:9 }
error: aborting due to previous error
