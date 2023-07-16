rust
mod test {
    pub trait A {
        fn a();
    }

    impl A for () {
        fn a() { }
    }
}

use test::A;

mod test2 {
    use super::*;
    pub fn b() {
        let _ = <()>::a();
    }
}

fn main() {
    test2::b();
}
