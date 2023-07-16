rust
struct PartiallyInitializedFoo { a: MaybeUninit<u8>, b: MaybeUninit<String> }
