
    trait Slice<T> {
        fn as_slice<'a>(&'a self) -> &'a [T];
        ...
    }

    trait MutSlice<T> { ... }
