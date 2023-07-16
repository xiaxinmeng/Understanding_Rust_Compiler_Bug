
<anon>:6:32: 6:42 error: the type of this value must be known in this context
<anon>:6     doit(0i, |&: x/*: int*/| { x.to_int(); });
                                        ^~~~~~~~~~
<anon>:6:5: 6:9 error: type mismatch: the type `closure` implements the trait `core::ops::Fn<([type error]),[type error]>`, but the trait `core::ops::Fn<(int),()>` is required (expected int, found type error)
<anon>:6     doit(0i, |&: x/*: int*/| { x.to_int(); });
             ^~~~
<anon>:6:5: 6:9 note: the trait `core::ops::Fn` must be implemented because it is required by `doit`
<anon>:6     doit(0i, |&: x/*: int*/| { x.to_int(); });
             ^~~~
error: aborting due to 2 previous errors
