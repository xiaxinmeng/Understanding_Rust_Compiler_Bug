 rust
struct Wrapper<T>(T);
trait Thing<W = Wrapper<Self::U>> { type U; }
fn main() {}
