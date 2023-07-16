rust
#[allow(non_upper_case_globals)]
const _IMPL_ARBITRARY_FOR_MyType: () = {
    impl proptest::arbitrary::Arbitrary for MyType {
        ...
    }
};
