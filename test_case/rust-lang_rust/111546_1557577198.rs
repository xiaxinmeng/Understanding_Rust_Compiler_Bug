rust
//! Sync version
use once_cell::sync::Lazy; // or `std::sync::LazyLock`

static USER_IDS: Lazy<HashMap<String, u32>> = Lazy::new(|| /* sync initialization code */);

fn johns_id() -> Option<u32> {
    // No `await` and no method call. `Lazy`'s `Deref` impl allows accessing the value directly.
    USER_IDS.get("John Doe").copied()
}
