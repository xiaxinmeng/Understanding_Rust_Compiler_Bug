rust
#![feature(staged_api)]
#![stable(feature = "stable_generic_feature", since = "1.40.0")]


#[stable(feature = "stable_generic_feature", since = "1.40.0")]
pub struct Foo<#[unstable(feature = "unstable_generic_feature", issue = "0")] T = usize> {
    _f: T,
}
