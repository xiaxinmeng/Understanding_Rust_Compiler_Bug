rs
#![feature(const_trait_impl)]
#![allow(incomplete_features)]

#![feature(const_fn_trait_bound)] // <- why is this needed? 

trait ConstDefaultFn: Sized {
    fn b(self);

    #[default_method_body_is_const]
    fn a(self) {
        self.b(); // Should not error here as this is only in `impl const ConstDefaultFn for Whatever`
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

const fn test() {
    NonConstImpl.a(); // Should error: `fn a(self) of NonConstImpl is not const`
    ConstImpl.a();
}

fn main() {}
