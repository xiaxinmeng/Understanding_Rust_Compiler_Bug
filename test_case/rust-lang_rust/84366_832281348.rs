rust
fn foo<'a>() -> &'a str { "" }
// 'desugars' to the rough equivalent of `struct foo<'a>; impl<'a> Fn<()> for foo<'a> { .. }`
