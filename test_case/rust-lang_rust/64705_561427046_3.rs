rust
macro_rules! perftools_inline {
    ($($item:tt)*) => (
        $($item)*
    );
}
pub struct RawFloatState;
impl RawFloatState {
    perftools_inline! {
        pub(super) fn new() {}
    }
}
