rs
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

use tokio::select;
use tokio::time::sleep;

fn recur() -> Pin<Box<dyn Future<Output = ()>>> {
    Box::pin(async {
        sleep(Duration::from_millis(10)).await;
        println!("Slept once");
        recur().await;
    })
}

#[tokio::main]
async fn main() {
    select! {
        () = recur() => unreachable!(),
        () = sleep(Duration::from_millis(100)) => println!("Done"),
    }
}
