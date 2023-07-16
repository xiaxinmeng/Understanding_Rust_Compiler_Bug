rust
#![deny(missing_debug_implementations)]

struct DontLookAtMe(i32);

async fn secret() -> DontLookAtMe {
    DontLookAtMe(41)
}

// Comment this function out to fix the lint...
pub async fn looking() -> i32 {
    secret().await.0
}

fn main() {
}
