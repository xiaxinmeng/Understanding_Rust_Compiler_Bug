 rust
macro_rules! assert_eq {
    ($left:expr , $right:expr) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    panic!("assertion failed: `(left == right)` \
                           (left: `{:?}`, right: `{:?}`)", left_val, right_val)
                }
            }
        }
    });
    ($left:expr , $right:expr, $msg:expr) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    panic!("assertion failed: `(left == right)` \
                            (left: `{:?}`, right: `{:?}`): {}", left_val, right_val, $msg)
                }
            }
        }
    });
    ($left:expr , $right:expr, $fmt:expr, $($arg:tt)*) => ({
        match (&($left), &($right)) {
            (left_val, right_val) => {
                if !(*left_val == *right_val) {
                    panic!(concat!("assertion failed: `(left == right)` \
                                   (left: `{:?}`, right: `{:?}`): ", $fmt),
                           left_val, right_val, $($arg)*)
                }
            }
        }
    });
}

// assert_eq!(1, 2);     >>> assertion failed: `(left == right)` (left: `1`, right: `2`)
// assert_eq!(1, 2, "Type is u32");
//                       >>> assertion failed: `(left == right)` (left: `1`, right: `2`): Type is u32
// assert_eq!(1, 2, "concurrency = {concurrency}, counter = {counter}",
//                   concurrency = c, counter = i);
//                       >>> assertion failed: `(left == right)` (left: `1`, right: `2`): concurrency = 8, counter = 65536
