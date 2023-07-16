rust
mod x {
    struct FakeString {}

    impl std::ops::Add<FakeString> for String {
        type Output = Self;
        fn add(mut self, rhs: FakeString) -> Self::Output {
            unreachable!()
        }
    }
}

mod y {
    pub fn hw() {
        println!("{}", "Hello".to_string() + &(" World".to_string()));
    }
}
