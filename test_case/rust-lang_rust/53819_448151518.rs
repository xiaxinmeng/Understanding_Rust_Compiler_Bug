rust
trait Foo<T> {
    const X: T;
}

trait Bar<U: Foo<usize>> {
    const F: u32 = [42][U::X];
}

struct Struct;

impl Foo<usize> for Struct {
    const X: usize = 99;
}

impl Bar<Struct> for Struct {}

fn main() {
    // comment out to not get any errors
    let x = Struct::F;
}
