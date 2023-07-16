rust
mod one {
    pub trait Tr {
        fn meth();
    }
}

mod two {
    use super::one::Tr;

    impl Tr for () {
        fn meth() {}
    }
}

use two::meth;
