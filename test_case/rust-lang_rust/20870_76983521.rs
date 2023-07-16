 rust
trait DebugCompare<RHS> {
    pub describe_difference(&self, rhs: &RHS) -> Option<Cow<str>>;
}

macro_rules! assert_eq {
    ($left:expr, $right:expr) => {{
        match (&($left), &($right)) {
            (left_val, right_val) => if let Some(desc) = left_val.describe_difference(right_val) {
                panic!("assert_eq! failed: difference between {:?} and {:?} is {}",
                    *left_val, *right_val, desc);
            },
        }
    }}
}
