
--------------------------------
1.0:

<source>:3:24: 3:30 error: mismatched types:
 expected `&_`,
    found `(_, _)`
(expected &-ptr,
    found tuple) [E0308]
<source>:3     [1, 2, 3].sort_by(|(x, y)| panic!());
                                                                                              ^~~~~~
<source>:3:15: 3:41 error: type mismatch: the type `[closure /tmp/compiler-explorer-compiler117326-8-ixlnde.do96qrggb9/example.rs:3:23: 3:40]` implements the trait `for<'r> core::ops::FnMut<(&'r _,)>`, but the trait `for<'r,'r> core::ops::FnMut<(&'r _, &'r _)>` is required (expected a tuple with 2 elements, found one with 1 elements) [E0281]
<source>:3     [1, 2, 3].sort_by(|(x, y)| panic!());
                                                                                     ^~~~~~~~~~~~~~~~~~~~~~~~~~
<source>:3:15: 3:41 error: type mismatch: the type `[closure /tmp/compiler-explorer-compiler117326-8-ixlnde.do96qrggb9/example.rs:3:23: 3:40]` implements the trait `for<'r> core::ops::FnOnce<(&'r _,)>`, but the trait `for<'r,'r> core::ops::FnOnce<(&'r _, &'r _)>` is required (expected a tuple with 2 elements, found one with 1 elements) [E0281]
<source>:3     [1, 2, 3].sort_by(|(x, y)| panic!());
                                                                                     ^~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to 3 previous errors
Compiler exited with result code 101

--------------------------------
1.11:

<source>:3:24: 3:30 error: mismatched types [E0308]
<source>:3     [1, 2, 3].sort_by(|(x, y)| panic!());
                                                                                               ^~~~~~
<source>:3:24: 3:30 help: run `rustc --explain E0308` to see a detailed explanation
<source>:3:24: 3:30 note: expected type `&_`
<source>:3:24: 3:30 note:    found type `(_, _)`
<source>:3:15: 3:22 error: type mismatch: the type `[closure@/tmp/compiler-explorer-compiler117326-8-1ggak45.5sng6wdn29/example.rs:3:23: 3:40]` implements the trait `for<'r> std::ops::FnMut<(&'r _,)>`, but the trait `for<'r, 'r> std::ops::FnMut<(&'r _, &'r _)>` is required (expected a tuple with 2 elements, found one with 1 elements) [E0281]
<source>:3     [1, 2, 3].sort_by(|(x, y)| panic!());
                                                                                      ^~~~~~~
<source>:3:15: 3:22 help: run `rustc --explain E0281` to see a detailed explanation
<source>:3:15: 3:22 error: type mismatch: the type `[closure@/tmp/compiler-explorer-compiler117326-8-1ggak45.5sng6wdn29/example.rs:3:23: 3:40]` implements the trait `for<'r> std::ops::FnOnce<(&'r _,)>`, but the trait `for<'r, 'r> std::ops::FnOnce<(&'r _, &'r _)>` is required (expected a tuple with 2 elements, found one with 1 elements) [E0281]
<source>:3     [1, 2, 3].sort_by(|(x, y)| panic!());
                                                                                      ^~~~~~~
<source>:3:15: 3:22 help: run `rustc --explain E0281` to see a detailed explanation
error: aborting due to 3 previous errors
Compiler exited with result code 101

--------------------------------
1.12 - 1.18nightly:

error[E0308]: mismatched types
 --> <source>:3:24
  |
3 |     [1, 2, 3].sort_by(|(x, y)| panic!());
  |                        ^^^^^^ expected &{integer}, found tuple
  |
  = note: expected type `&{integer}`
  = note:    found type `(_, _)`
error[E0281]: type mismatch: the type `[closure@<source>:3:23: 3:40]` implements the trait `for<'r> std::ops::FnMut<(&'r {integer},)>`, but the trait `for<'r, 'r> std::ops::FnMut<(&'r {integer}, &'r {integer})>` is required (expected a tuple with 2 elements, found one with 1 elements)
 --> <source>:3:15
  |
3 |     [1, 2, 3].sort_by(|(x, y)| panic!());
  |               ^^^^^^^
error[E0281]: type mismatch: the type `[closure@<source>:3:23: 3:40]` implements the trait `for<'r> std::ops::FnOnce<(&'r {integer},)>`, but the trait `for<'r, 'r> std::ops::FnOnce<(&'r {integer}, &'r {integer})>` is required (expected a tuple with 2 elements, found one with 1 elements)
 --> <source>:3:15
  |
3 |     [1, 2, 3].sort_by(|(x, y)| panic!());
  |               ^^^^^^^
error: aborting due to 3 previous errors
Compiler exited with result code 101

--------------------------------
