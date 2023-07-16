rust
type T<'x> = Box<dyn Bar + 'x>;

trait Bar {
    fn bar(self, arg: T);
}

impl Bar for T<'_> {
    fn bar(self, arg: T) {
        arg = self;
    }
}
