 rust
enum List<T> {
    Cons(T, ~List<T>),
    Nil
}

fn main() {
  println(format!("{}", std::mem::size_of::<List<int>>()));
}
