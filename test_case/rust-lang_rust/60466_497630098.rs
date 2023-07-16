
struct S<T: Debug, const N: usize>([T; N]); //~ ERROR `[T; _]` doesn't implement `std::fmt::Debug`
