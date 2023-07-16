rust
#![feature(const_loop)]

async fn fun() {
    [1; ().await];
}
