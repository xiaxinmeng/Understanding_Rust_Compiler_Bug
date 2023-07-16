rust
macro_rules! produce_it {
    ($expr:expr) => {
        #[derive(Print)]
        struct Foo { 
            val: [bool; $expr]
        }
    }
}

produce_it!({ #![cfg_attr(not(FALSE), allow(unused))] 30 });
