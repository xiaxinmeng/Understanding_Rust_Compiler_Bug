
mod impl_two {
    use super::upstream::*;
    struct TwoS ;
    impl Foo<TwoS> for String {
        fn execute(self) -> TwoS {
            TwoS {}
        }
    }
}
