rust
use tokio::task;

struct StructA {
    b: StructB,
}

impl StructA {
    async fn foo(&self) {
        let bar = self.b.bar().await;
        task::spawn_blocking(move || {
            self.b;
        });
    }
}

struct StructB {}

impl StructB {
    async fn bar(&self) -> Option<u8> {
        None
    }
}
