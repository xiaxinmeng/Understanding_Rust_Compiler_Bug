 rust
#[allow(deprecated)]
macro_rules! macro_test_allow(
    () => (deprecated());
)

macro_test_allow!();
