rust
#![feature(staged_api)]

#![unstable(feature = "rustc-deprecated-test", reason = "reason", issue = "1")]

// @has rustc_deprecated/constant.FOO.html '//*[@class="stab deprecated"]' \
//      'Deprecated since 1.2.3: because I said so'
#[rustc_deprecated(since = "1.2.3", reason = "because I said so")]
#[stable(feature = "rust1", since = "1.0.0")]
pub const FOO: usize = 1;

// @has rustc_deprecated/static.BAR.html '//*[@class="stab deprecated"]' \
//      'Deprecating in 2.0.0: no reason'
#[rustc_deprecated(since = "2.0.0", reason = "no reason")]
#[stable(feature = "rust1", since = "1.0.0")]
pub static BAR: usize = 1;
