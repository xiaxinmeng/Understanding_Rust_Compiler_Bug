rust
macro_rules! foo {
    () => { true; }
}

async fn func() {
    let _ = foo!();
}

fn main() {}
