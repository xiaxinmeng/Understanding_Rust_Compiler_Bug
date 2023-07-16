rust
use lightning_net_tokio::Connection;

#[tokio::main]
async fn main() {
        tokio::spawn(async move {
                Connection::setup_outbound().await;
        });
}
