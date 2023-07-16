
trait Trait {
    fn outer(self) {
        fn inner(_: Self) {
        }
    }
}

fn main() { }
