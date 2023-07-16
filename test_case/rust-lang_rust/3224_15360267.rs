
struct X { v: int }

impl Drop for X {
    fn finalize(&self) { io::println(fmt!("%?", ptr::to_uint(self))); }
}

fn main() {
    let x = X { v: 42 };
    let &y = &x;
}
