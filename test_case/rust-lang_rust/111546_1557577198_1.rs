rust
//! Async version
use async_lazy::Lazy; // or `async_once_cell::Lazy`

static USER_IDS: Lazy<HashMap<String, u32>> = Lazy::const_new(|| Box::pin(async {/* async initialization code */}));

fn johns_id_no_autoref() -> Option<u32> {
    // `.await` and extra method call. +14 characters compared to sync version of this code
    USER_IDS.force().await.get("John Doe").copied()
}

fn johns_id_with_autoref() -> Option<u32> {
    // `.await`, but no extra method call. Much closer to sync version, +6 characters
    USER_IDS.await.get("John Doe").copied()
}
