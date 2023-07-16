rust
macro_rules! maybe_zero {
    ($t:type) => {
        if MyTrait::can_zero_init::<$t>() {
            Some(std::mem::zeroed::<$T>())
        } else {
            None
        }
    }
}
