 rust
struct Bar { _x: [u8] }

impl Drop for Bar {
    fn drop(&mut self) {
        println!("dropping")
    }
}

fn main() {
    let _x: Box<Bar> = unsafe { std::mem::transmute(Box::new([]) as Box<[u8]>) };
}
