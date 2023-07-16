 rust
trait FooUtil {
    fn add(self) -> int;
}

impl FooUtil for char {
    fn add(self) -> int {
        2
    }
}

fn main() {
    let a  = 'c';
    a.add();
}
