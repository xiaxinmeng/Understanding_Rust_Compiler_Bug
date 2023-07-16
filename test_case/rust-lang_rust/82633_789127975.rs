rust
#![feature(trivial_bounds)]
#![feature(unboxed_closures)]

trait A { fn a() where Self: Sized; }
impl A for str {
    fn a() where Self: Sized {
        unsafe { std::hint::unreachable_unchecked() }
    }
}

fn foo<F: FnOnce<()>>() where F::Output: A { F::Output::a() }

fn main() { foo::<fn() -> str>() }
