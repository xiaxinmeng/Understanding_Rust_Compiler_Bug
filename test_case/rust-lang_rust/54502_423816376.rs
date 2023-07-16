rust
declare_lint! {
    OVERFLOWING_LITERALS,
    Warn,
    "literal out of range for its type",
    Edition::Edition2018 => Deny
}
