 rust
struct Dropper {
    name: &'static str
}

impl Drop for Dropper {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}

pub fn main() {
    let _a: Dropper;
    let _b: Dropper = Dropper { name: "b" };
    _a = Dropper { name: "a" };
    println!("main");
}
