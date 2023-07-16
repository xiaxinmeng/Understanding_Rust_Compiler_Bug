rust
async fn run_1<'a>() {
    let data = &mut [0u8; { 1 + 4 }];
    run_2().await
}

async fn run_2() {}
fn main() {}
