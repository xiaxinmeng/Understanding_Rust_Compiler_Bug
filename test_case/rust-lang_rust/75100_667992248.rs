rust
#[cfg(any(windows, doc))]
fn this() -> u32 {
    3
}

#[cfg(any(unix, doc))]
fn this() -> u32 {
    4
}

pub async fn wow() -> u32 {
    this()
}
