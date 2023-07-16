
<anon>:1:1: 7:2 error: the trait `core::marker::Sized` is not implemented for the type `core::convert::AsRef<str>` [E0277]
<anon>:1 pub fn str_join(strs: &[AsRef<str>], join: str) -> String {
<anon>:2     strs.iter().fold(String::new(), |mut acc, s| {
<anon>:3         if !acc.is_empty() { acc.push_str(join); }
<anon>:4         acc.push_str(s);
<anon>:5         acc
<anon>:6     });
         ...
<anon>:1:1: 7:2 help: see the detailed explanation for E0277
<anon>:1:1: 7:2 note: `core::convert::AsRef<str>` does not have a constant size known at compile-time
<anon>:1:1: 7:2 note: slice and array elements must have `Sized` type
error: aborting due to previous error
