
mod impl_one {
    use super::upstream::*;
    struct OneS;
    impl <O> Foo<O> for String
        where O == OneS {
        fn execute(self) -> OneS {
            OneS {}
        }
    }
}
