 rust
impl BuildHasher for RandomState {
    type Hasher = DefaultHasher;
    // ...
}

pub struct DefaultHasher(SipHasher);

// ...
