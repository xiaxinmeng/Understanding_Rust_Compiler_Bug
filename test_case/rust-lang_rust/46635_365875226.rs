rust
pub mod some_module:: {
    pub macro a() {
        ::some_module::private_fn();
    }

    pub macro b($n:ident) {
        ::some_module::$n();
    }

    pub macro c($n:path) {
        $n();
    }

    pub macro d($n:expr) {
        $n;
    }

    fn private_fn() { }
}

pub fn main() {
    some_module::a!();
    some_module::b!(private_fn);
    some_module::c!(::some_module::private_fn);
    some_module::d!(::some_module::private_fn());
}
