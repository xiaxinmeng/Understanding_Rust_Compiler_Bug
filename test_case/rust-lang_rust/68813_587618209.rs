rust
use lightning_net_tokio::setup_outbound;

async fn f() {
    tokio::spawn(async move {
        setup_outbound().await;
    });
}

fn main() {}
