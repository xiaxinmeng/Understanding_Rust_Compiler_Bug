rust
use std::future::Future; //would be necessary to call `poll` if the receiver type was correct

async fn f() {
    
}

fn foo(cx: &mut std::task::Context) {
    f().poll(cx);
}
