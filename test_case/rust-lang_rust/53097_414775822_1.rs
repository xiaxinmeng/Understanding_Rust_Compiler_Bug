rust
extern crate pretty_assertions;

use pretty_assertions::assert_eq;

fn a() {
    assert_eq!(0, 0);
}

mod m {
    fn b() {
        assert_eq!(0, 0);
    }

    mod m1 {
        use pretty_assertions::assert_eq;

        #[test]
        fn c() {
            assert_eq!(0, 0, "");
        }
    }
}
