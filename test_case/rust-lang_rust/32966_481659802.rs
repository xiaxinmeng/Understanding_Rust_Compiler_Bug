rust
pub fn bar() {
    let x = BigTest::new();
}

pub struct BigTest {
    arr: [u32; 128]
}

impl BigTest {
    pub fn new() -> BigTest {
        BigTest {
            arr: [123; 128],
        }
    }
}
