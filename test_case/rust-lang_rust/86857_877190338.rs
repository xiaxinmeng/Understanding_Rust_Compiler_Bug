rust
trait ConstDefaultFn: Sized {
    fn b(self);
    fn a(self);
}

default impl<T> const ConstDefaultFn for T {
    fn a(self) {
        self.b();
    }
}

struct NonConstImpl;
struct ConstImpl;

impl ConstDefaultFn for NonConstImpl {
    fn b(self) {}
}

impl const ConstDefaultFn for ConstImpl {
    fn b(self) {}
}
