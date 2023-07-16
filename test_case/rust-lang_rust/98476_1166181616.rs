rust
// build-pass
// edition:2018
// compile-flags: -Zdrop-tracking=y

fn main() {
    let _ = foo();
}

async fn from_config(x: Config) {
    async {}.await;
    drop(x);
}

async fn foo() {
    from_config(Config {
        nickname: NonCopy,
        ..Default::default()
    })
    .await;
}

#[derive(Default)]
struct NonCopy;
impl Drop for NonCopy {
    fn drop(&mut self) {}
}

#[derive(Default)]
struct Config {
    nickname: NonCopy,
}
