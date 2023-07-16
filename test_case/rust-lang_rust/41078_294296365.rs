rust
fn main() {
    let suitable = |_| true;
    vec![1,2,3].into_iter().find(suitable);
}
