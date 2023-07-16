rust
struct Print(&'static str);

impl Drop for Print {
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}

fn a() -> Print { Print("a") }
fn b() -> Print { Print("b") }

fn main() {
    loop {
        std::mem::forget(({a()}, b(), Print("c"), break));
    }
}
