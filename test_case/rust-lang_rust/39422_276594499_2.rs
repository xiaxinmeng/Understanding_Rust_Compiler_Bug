rust
macro_rules! slice_compare (
    ($a:expr, $b:expr, $len:expr) => {{
        match $len {
            1 => cmp!($a, $b, u8, 0),
            2 => cmp!($a, $b, u16, 0),
            3 => cmp!($a, $b, u16, 0) && cmp!($a, $b, u8, 2),
            4 => cmp!($a, $b, u32, 0),
            ....
