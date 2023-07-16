rust
#![feature(staged_api)]
#![stable(feature = "stable_test_feature", since = "1.0.0")]

#[unstable(feature = "unstable_test_feature", issue = "none")]
pub enum Foo {
	...
}
