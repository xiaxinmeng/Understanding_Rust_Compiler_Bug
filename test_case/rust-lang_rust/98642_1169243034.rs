rust
fn main() {}

trait A {
    fn a(aa: B) -> Result<_, B> {
        Ok(())
    }
}

enum B {}
