 rust
struct Simple { data: u8 }
struct Complex<T> { data: T }

fn main() {
  let s = Simple{ data: 42 };
  processing( s );
}

fn processing(s : Complex) {}
//fn processing<T>(s : Complex<T>) {}
