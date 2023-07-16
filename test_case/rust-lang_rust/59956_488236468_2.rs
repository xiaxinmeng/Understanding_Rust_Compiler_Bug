rust
struct A<T, U = Self>(T, Option<Box<U>>);

struct B<T>(T, Option<Box<B<T>>>);
