rust
trait Has {
    fn has() {}
}

trait HasNot {}

impl Has for dyn HasNot {}

fn main() {
    <dyn HasNot>::has();
}
