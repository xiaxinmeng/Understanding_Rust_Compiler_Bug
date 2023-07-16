
use std::io::Error;

fn main() {
    let server = async {
        for _connection in 0..2 {
            futures::future::ready(Ok::<(), Error>(())).await?;
        }

        Ok::<(), Error>(())
    };

    server.await;
}
