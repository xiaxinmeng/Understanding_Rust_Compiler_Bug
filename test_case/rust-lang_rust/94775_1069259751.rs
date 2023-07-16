rust
struct Print(&'static str);

impl Drop for Print {
    fn drop(&mut self) {
        println!("{}", self.0);
    }
}

const A: Print = Print("a");
const B: Print = Print("b");

fn main() {
    loop {
        std::mem::forget(({A}, B, Print("c"), break));
    }
}
