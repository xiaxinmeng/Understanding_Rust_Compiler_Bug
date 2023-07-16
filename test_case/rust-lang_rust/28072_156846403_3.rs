
const C: usize = 42;

macro_rules! mac {
    ( $e:expr ) => {
        mod a_test_mod {
            #[test]
            fn funky() {
                println!("inside funky(), e is: {}", $e);
            }
        }
    }
}

mac!(super::C);
