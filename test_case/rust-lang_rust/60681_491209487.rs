rust
struct MyTest {
    a: &'static [u64],
}

fn main() -> Result<(), ()> {
    let b = MyTest {
        #[allow(const_err)]
        a: &[0 - 1],
    };

    Ok(())
}
