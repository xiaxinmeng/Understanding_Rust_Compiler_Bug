rust
impl Get for i32 {
    fn get() -> Self { 42 }
}

impl Use for &'static i32 {
    fn use_up(self) {
        std::thread::spawn(move || println!("{}", self));
    }
}
