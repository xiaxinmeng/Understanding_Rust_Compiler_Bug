rust
#![unstable(feature = "humans",
            reason = "who ever let humans program computers,
            we're apparently really bad at it",
            issue = "0")]

#![feature(rustc_const_unstable, const_fn, foo, foo2)]
#![feature(min_const_unsafe_fn)]
#![feature(staged_api)]

#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_const_unstable(feature="foo")]
const unsafe fn foo() -> u32 { 42 }

#[unstable(feature = "rust1", issue="0")]
const fn foo2() -> u32 { 42 }

#[stable(feature = "rust1", since = "1.0.0")]
const fn bar2() -> u32 { 42 }

#[unstable(feature = "foo2", issue="0")]
const unsafe fn foo2_gated() -> u32 { 42 }

#[stable(feature = "rust1", since = "1.0.0")]
const unsafe fn bar2_gated() -> u32 { 42 }
