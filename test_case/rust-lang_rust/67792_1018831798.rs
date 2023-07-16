rust
#![feature(const_trait_impl)]
#![feature(const_fn_trait_bound)]

trait Other {}
impl Other for u32 {}

trait Test {
    #[default_method_body_is_const]
    fn method<T: ~const Other, F: ~const FnOnce(T) -> bool>(f: F, x: T) -> bool {
        f(x)
    }
}

struct NonConst;
impl Test for NonConst {
    fn method<T: Other, F: FnOnce(T) -> bool>(f: F, x: T) -> bool {
        f(x)
    }
}

struct Const;
impl const Test for Const {
    fn method<T: ~const Other, F: ~const FnOnce(T) -> bool>(f: F, x: T) -> bool {
        f(x)
    }
}

fn main() {
    Const::method(|x| x > 0, 1);
}
