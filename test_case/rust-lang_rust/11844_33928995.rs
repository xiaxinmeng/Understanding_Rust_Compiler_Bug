 rust
enum A { B }
enum C<T> { D(T) }

fn main() {
  match B {
    D(a) => format!("{}", a)
  };
}
