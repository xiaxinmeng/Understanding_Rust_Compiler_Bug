 rust
#![feature(staged_api)]
#![staged_api]

#![unstable(feature = "foo")]

extern {
    pub fn foo();
}
