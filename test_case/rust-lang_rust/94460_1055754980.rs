rust
struct A;
impl Drop for A { fn drop(&mut self) {} }

pub async fn f() {
    let mut a = A;
    a = A;
    drop(a);
    async {}.await;
}
