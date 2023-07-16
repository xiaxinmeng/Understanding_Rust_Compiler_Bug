rust
#![feature(async_await)]

async fn print_dur() {}

fn main() -> () {
    async{
        let task1 = print_dur().await;
    }.await
}
