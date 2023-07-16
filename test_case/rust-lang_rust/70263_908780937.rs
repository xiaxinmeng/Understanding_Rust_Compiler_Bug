
trait A<'a>: Sized {
    fn get_value(data: &'a i32) -> Self;
}

impl<'a> A<'a> for &'a i32 {
    fn get_value(data: &'a i32) -> Self {
        data
    }
}

fn do_something(n: &i32) {}

fn wrap<F, T>(f: F)
where
    F: Fn(T),
     T: for<'a> A<'a>,
{
    let n = 100;
    f(T::get_value(&n));
}


fn main() {
    wrap(do_something);
}
