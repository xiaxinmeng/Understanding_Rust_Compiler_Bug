 rust
trait Expression {}

trait Column {}

impl<C: Column> Expression for C {}


fn assert_expression<T: Expression>() {}

fn main() {
    assert_expression::<()>();
}
