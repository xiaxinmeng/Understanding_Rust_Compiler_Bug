rust
struct Pika {
    a: Option<u32>,
}

impl Pika {
    pub fn get_a(&self) -> Option<u32> {
        self.a.cloned()
    }
}

fn main() {
    let my_pika = Pika { a: Some(12) };
    println!("{:?}", my_pika.get_a());
}
