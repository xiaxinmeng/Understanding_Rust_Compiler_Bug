rust
// you can unsize `U` without this feature, but
struct A<T, U: ?Sized + 'static>(T, T, U);

// after extracting the last 2 fields, you cannot unsize `U` anymore.
struct A<T, U: ?Sized + 'static>(T, B<T, U>);
struct B<T, U: ?Sized + 'static>(T, U);
