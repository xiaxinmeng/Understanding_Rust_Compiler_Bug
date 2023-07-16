rust
mod sub1 {
    // this should be shadowed by sub2::describe
    /// sub1::describe
    pub fn describe() -> &'static str {
        "sub1::describe"
    }

    // this should be shadowed,
    // because both sub1::describe2 and sub3::describe2 are from glob reexport
    /// sub1::describe2
    pub fn describe2() -> &'static str {
        "sub1::describe2"
    }
}

mod sub2 {
    /// sub2::describe
    pub fn describe() -> &'static str {
        "sub2::describe"
    }
}

mod sub3 {
    // this should be shadowed
    // because both sub1::describe2 and sub3::describe2 are from glob reexport
    /// sub3::describe2
    pub fn describe2() -> &'static str {}
}

// ...

#[doc(inline)]
pub use sub2::describe;

#[doc(inline)]
pub use sub1::*;

#[doc(inline)]
pub use sub3::*;
