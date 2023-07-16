rust
#[cfg(any(feature="long-feature-flag-name",feature="appservice-api", not(feature="clientservice-c"), all(feature="clientservice-rust",feature="identity-clientservice-embedded-something-something-i-need-something-long-to-testing")))]
#[no_mangle]
/// This is a function that requires a lot of flags
pub fn rarely_used_function() {}
