rust
#![allow(incomplete_features)]
#![feature(const_generics)]

async fn test(test: [(); { 0 }]) {
    let _ = &test;
    async {}.await;
}
