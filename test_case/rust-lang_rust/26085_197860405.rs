 rust
// This prints "default apply"
// rustc 1.9.0-nightly (c66d2380a 2016-03-15)
#![feature(specialization)]

pub fn apply<F>(f: F)
    where F: FnMut()
{
    <()>::apply(f);
}

trait Apply<F> {
    fn apply(f: F);
}

impl<F> Apply<F> for ()
    where F: FnMut()
{
    default fn apply(f: F) {
        println!("default apply")
    }
}

impl<F> Apply<F> for ()
    where F: Fn()
{
    fn apply(f: F) {
        println!("thread safe apply")
    }
}


fn main() {
    apply(|| {});
}

