rust
 ($left:expr, $right:expr) => ({
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    // The reborrows below are intentional. Without them, the stack slot for the
                    // borrow is initialized even before the values are compared, leading to a
                    // noticeable slow down.
                    panic!(concat!(r#"assertion failed: `(", stringify!($left), ") == (", stringify!($right), ")`",
  left: `{:?}`,
 right: `{:?}`"#), &*left_val, &*right_val)
                }
            }
        }
    });
