rust
fn main() {
    if !(foo(&[], 10) == (&[], 0)) {
        panic!();
    };

    match (&foo(&[], 10), &(&[], 0)) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                panic!()
            }
        }
    };
}
