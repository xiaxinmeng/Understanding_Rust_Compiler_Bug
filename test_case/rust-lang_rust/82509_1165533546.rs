rust
struct Evaluatable<const N: i32>;

... where Evaluatable<{ my_expr }>: Sized
