rust
struct Test(String);

impl Test {
    async fn borrow_async(&self) {}

    fn borrow(&self) {}

    fn with(&mut self, s: &str) -> &mut Self {
        self.0 = s.into();
        self
    }
}

async fn test() {
    // error[E0716]: temporary value dropped while borrowed
    Test("".to_string()).with("123").borrow_async().await;
}

fn main() {
    // Temporary outlives the borrow() call
    Test("".to_string()).with("123").borrow();
}
