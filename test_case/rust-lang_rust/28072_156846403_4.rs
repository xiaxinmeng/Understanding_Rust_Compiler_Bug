
const C: usize = 42;

macro_rules! define_tests {
    ( $name:ident $e:expr ) => {
        mod $name {
            #[test]
            fn test_one_way() {
                ...
            }

            #[test]
            fn test_another_way() {
                ...
            }
        }
    }
}

/* Run both kinds of tests on various boundary conditions: */
define_tests!(one_c, super::C);
define_tests!(one_less_than_c, super::C - 1);
define_tests!(one_more_than_c, super::C + 1);
define_tests!(two_c, super::C + super::C);
