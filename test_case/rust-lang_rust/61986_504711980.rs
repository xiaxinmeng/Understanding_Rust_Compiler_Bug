rust
#![feature(async_await)]

async fn bar() -> Option<()> {
    Some(())
}

async fn listen() {
    while let Some(_) = bar().await {
        String::new();
    }
}

fn main() {
    listen();
}
