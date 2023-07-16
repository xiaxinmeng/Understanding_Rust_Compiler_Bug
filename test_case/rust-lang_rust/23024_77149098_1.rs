
<anon>:5:26: 5:28 error: angle-bracket notation is not stable when used with the `Fn` family of traits, use parentheses [E0215]
<anon>:5     let f = vfnfer[0] as Fn;
                                  ^~
<anon>:5:26: 5:28 help: add `#![feature(unboxed_closures)]` to the crate attributes to enable
<anon>:5     let f = vfnfer[0] as Fn;
                                  ^~
<anon>:5:26: 5:28 error: wrong number of type arguments: expected 1, found 0 [E0243]
<anon>:5     let f = vfnfer[0] as Fn;
                                  ^~
<anon>:5:13: 5:22 error: internal compiler error: expected object type
<anon>:5     let f = vfnfer[0] as Fn;
                     ^~~~~~~~~
