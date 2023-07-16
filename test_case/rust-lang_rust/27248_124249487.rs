 rust
trait T:'static {}
fn main() {
    fn _message_down_cast(_m: &T) -> &i8 {
        panic!()
    }
}
