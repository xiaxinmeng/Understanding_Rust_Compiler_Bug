rust
use std::future::Future;

fn main() {
    let _ = wrapper(hello);
}

static GLOBAL: i32 = 42;

async fn wrapper<'a, F, FutResp>(f: F)
where
    F: Fn(&'a i32) -> FutResp + 'a,
    FutResp: Future<Output = ()>,
{
    f(&GLOBAL).await
}

async fn hello(_: &i32) {
    println!("Hello");
}
