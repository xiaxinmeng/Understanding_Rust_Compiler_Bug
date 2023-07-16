rust
struct NotSync(*const ());

async fn async_not_sync() {
    let a = NotSync(0 as *const ());
    // Uncomment this line to see a better error
    //let b: *const ();
    
    // Force 'a' to be live across a yield point
    async fn dummy() {}
    dummy().await;
}

fn require_sync<T: Sync>(val: T) {}

fn main() {
    require_sync(async_not_sync());
}
