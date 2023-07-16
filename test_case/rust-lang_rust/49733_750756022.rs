
#[allow_internal_unstable(box_syntax)]
macro_rules! boxed {
    ($item: expr) => {{
        box ($item)
    }}
}
