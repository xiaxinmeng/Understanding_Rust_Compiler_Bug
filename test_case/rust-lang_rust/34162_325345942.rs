rust
fn main() {
    let mut a = [1,3,2];
    a.sort_by_key(|x| x);
    a.my_sort_by_key(|x| x);
}
