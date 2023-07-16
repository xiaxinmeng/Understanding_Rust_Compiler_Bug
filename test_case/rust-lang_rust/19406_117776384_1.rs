 rust
<anon>:2:43: 2:52 error: no method named `collect` found for type `core::str::Split<'_, u8>` in the current scope
<anon>:2     let v: Vec<_> = "1, 2, 3".split(b',').collect();
                                                   ^~~~~~~~~
<anon>:2:43: 2:52 note: the method `collect` exists but the following trait bounds were not satisfied: `u8 : core::str::pattern::Pattern<'static>`, `core::str::Split<'_, u8> : core::iter::Iterator`
<anon>:2:31: 2:42 error: the trait `core::ops::FnMut<(char,)>` is not implemented for the type `u8` [E0277]
<anon>:2     let v: Vec<_> = "1, 2, 3".split(b',').collect();
                                       ^~~~~~~~~~~
<anon>:2:31: 2:42 error: the trait `core::ops::FnOnce<(char,)>` is not implemented for the type `u8` [E0277]
<anon>:2     let v: Vec<_> = "1, 2, 3".split(b',').collect();
