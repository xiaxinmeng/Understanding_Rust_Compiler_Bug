rust
#![feature(staged_api, const_fn)]
#![stable(feature = "repro", since = "0")]
#![allow(dead_code)]

#[unstable(feature = "detail", issue = "none")]
pub mod detail {
    #[rustc_const_stable(...)]
    pub(crate) const fn bar() {}
}

// not public, not rustc_const_stable, maybe not even used
const fn foo() {
    detail::bar();
}
