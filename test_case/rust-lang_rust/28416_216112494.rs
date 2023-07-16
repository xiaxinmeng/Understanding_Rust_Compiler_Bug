
<anon>:3:85: 3:87 error: the type of this value must be known in this context
<anon>:3     let indices = buf.iter().zip(buf.iter().skip(1)).enumerate().filter(|_, (a, b)| *a == b'\r' && *b == b'\n').map(|(i, (_, _))| i);
                                                                                             ^~
<anon>:3:100: 3:102 error: the type of this value must be known in this context
<anon>:3     let indices = buf.iter().zip(buf.iter().skip(1)).enumerate().filter(|_, (a, b)| *a == b'\r' && *b == b'\n').map(|(i, (_, _))| i);
                                                                                                            ^~
<anon>:3:113: 3:116 error: no method named `map` found for type `core::iter::Filter<core::iter::Enumerate<core::iter::Zip<core::slice::Iter<'_, u8>, core::iter::Skip<core::slice::Iter<'_, u8>>>>, [closure@<anon>:3:73: 3:111]>` in the current scope
<anon>:3     let indices = buf.iter().zip(buf.iter().skip(1)).enumerate().filter(|_, (a, b)| *a == b'\r' && *b == b'\n').map(|(i, (_, _))| i);
                                                                                                                         ^~~
<anon>:3:113: 3:116 note: the method `map` exists but the following trait bounds were not satisfied: `[closure@<anon>:3:73: 3:111] : core::ops::FnMut<(&(usize, (&_, &_)),)>`, `core::iter::Filter<core::iter::Enumerate<core::iter::Zip<core::slice::Iter<'_, u8>, core::iter::Skip<core::slice::Iter<'_, u8>>>>, [closure@<anon>:3:73: 3:111]> : core::iter::Iterator`
<anon>:3:66: 3:72 error: type mismatch: the type `[closure@<anon>:3:73: 3:111]` implements the trait `for<'r> core::ops::FnMut<(&'r (usize, (&u8, &u8)), ([type error], [type error]))>`, but the trait `for<'r> core::ops::FnMut<(&'r (usize, (&u8, &u8)),)>` is required (expected a tuple with 1 elements, found one with 2 elements) [E0281]
<anon>:3     let indices = buf.iter().zip(buf.iter().skip(1)).enumerate().filter(|_, (a, b)| *a == b'\r' && *b == b'\n').map(|(i, (_, _))| i);
                                                                          ^~~~~~
<anon>:3:66: 3:72 help: see the detailed explanation for E0281
<anon>:3:66: 3:72 error: type mismatch: the type `[closure@<anon>:3:73: 3:111]` implements the trait `for<'r> core::ops::FnOnce<(&'r (usize, (&u8, &u8)), ([type error], [type error]))>`, but the trait `for<'r> core::ops::FnOnce<(&'r (usize, (&u8, &u8)),)>` is required (expected a tuple with 1 elements, found one with 2 elements) [E0281]
<anon>:3     let indices = buf.iter().zip(buf.iter().skip(1)).enumerate().filter(|_, (a, b)| *a == b'\r' && *b == b'\n').map(|(i, (_, _))| i);
                                                                          ^~~~~~
<anon>:3:66: 3:72 help: see the detailed explanation for E0281
