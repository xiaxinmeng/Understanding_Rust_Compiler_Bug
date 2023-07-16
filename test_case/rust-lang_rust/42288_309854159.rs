
#[compile_fail]
fn bad() {
    fn my_clone<T>(v: Vec<T>) -> Vec<T> {
        v.clone()
    }
}
