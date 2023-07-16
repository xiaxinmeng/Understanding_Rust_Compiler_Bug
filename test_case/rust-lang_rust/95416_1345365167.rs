rs
use tokio::sync::Mutex;
use std::sync::Arc;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref MUTEX_MAP: Mutex<HashMap<String, Arc<Mutex<()>> >> = Mutex::new(HashMap::new());
}

async fn acquire_guard(_dir: &str) -> Arc<Mutex<()>> {
    let mut mapGuard = MUTEX_MAP.lock().await;
    if !mapGuard.contains_key(_dir) {
        let mutex = Arc::new(Mutex::new(()));
        mapGuard.insert(_dir.to_string(), mutex);
    }
    mapGuard.get(_dir).unwrap().clone()
}

#[tokio::main]
async fn main() {
    let n = "test";
    let mutex = acquire_guard(n).await;
    {
        let _guard = mutex.lock().await;
    } // guard is dropped
}
