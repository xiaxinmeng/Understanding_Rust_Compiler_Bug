rust
trait Foo {
    fn my_method(&self) {}
}

impl Foo for bool {
    fn my_method(&self) {
        std::process::abort();
    }
}

#[inline(never)]
pub fn use_it(val: &dyn Foo) {
    val.my_method();
}

pub fn main() {
    use_it(&true);
}
