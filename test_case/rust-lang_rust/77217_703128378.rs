rust
#![warn(single_use_lifetimes)]
pub async fn foo<'a>(x: &'a u32) -> &'a u32 {
	x
}
