rust
#[derive(Default)]
#[rustc_insignificant_dtor]
pub struct Client2;
impl Drop for Client2 {
    fn drop(&mut self) {}
}

fn main() {
    let f = move || match drop(Client2 { ..Client2::default() }) {
        _status => yield,
    };
    assert_send(f);
}
