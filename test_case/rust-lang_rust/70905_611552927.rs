
// build-pass
// edition:2018

async fn test() {
    const C: usize = 4;
    foo(&mut [0u8; C]).await;
}

async fn foo(bs: &mut [u8]) {}

fn main() {}
