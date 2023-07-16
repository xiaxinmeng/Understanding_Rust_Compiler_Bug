rust
#[cfg(any(target_os = "macos", target_os = "ios"))]
#[error("OSX creatively eats your data, using Lightning on OSX is unsafe")]
struct ERR {}
