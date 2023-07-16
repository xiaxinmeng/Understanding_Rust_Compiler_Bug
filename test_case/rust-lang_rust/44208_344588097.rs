rust
struct Container<T> {
    inner: T,
}

impl<T> Container<&'static T> {
    fn do_something(&self) {
        println!("Container<&'a T>::do_something");
    }
}

impl<'a, T> Container<&'a mut T> {
    fn do_something(&self) {
        println!("Container<&'a mut T>::do_something");
    }
}

trait Issue<T> {
    fn make_container(&self) -> Container<T>;
}

impl<T> Issue<&'static T> for &'static T {
    fn make_container(&self) -> Container<&'static T> {
        Container { inner: &self }
    }
}

impl Issue<()> for &'static char {
    fn make_container(&self) -> Container<()> {
        Container { inner: () }
    }
}

fn main() {
    static ARR: &char = &'a';
    let _ = ARR.make_container().do_something();
}
