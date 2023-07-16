rust
fn need_send<T: ?Sized + Send>() {}

fn main() {
    need_send::<dyn Send>();
}
