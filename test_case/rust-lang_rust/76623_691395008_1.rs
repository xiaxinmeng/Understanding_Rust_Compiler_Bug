rust
// @has 'foo/fn.foo2.html' '//pre' 'pub fn foo2() -> u32'
#[unstable(feature = "humans", issue = "none")]
pub const fn foo2() -> u32 { 42 }


// @has 'foo/fn.foo2_gated.html' '//pre' 'pub unsafe fn foo2_gated() -> u32'
#[unstable(feature = "foo2", issue = "none")]
pub const unsafe fn foo2_gated() -> u32 { 42 }

// @has 'foo/fn.bar_not_gated.html' '//pre' 'pub unsafe fn bar_not_gated() -> u32' 
pub const unsafe fn bar_not_gated() -> u32 { 42 }
