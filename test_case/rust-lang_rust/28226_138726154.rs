 rust
struct Mapped<I, F> {
  iter: I,
  func: F
}

struct Compose<F, G>(F, G);

fn map<I, T, U, F, F2, U2>(m: Mapped<I, F>, func: F2) -> Mapped<I, Compose<F, F2>>
  where I: Iterator<Item=T>,
            F: Fn(T) -> U,
            F2: Fn(U) -> U2 {
  Mapped { iter: m.iter, func: Compose(m.func, func) }
}
