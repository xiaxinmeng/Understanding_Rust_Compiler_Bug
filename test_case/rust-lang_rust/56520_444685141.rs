rust
enum Test { }

impl Test {
    fn ice(&self) {
        match &Self {
            _ => { println!("ICE"); }
        }
    }
}
