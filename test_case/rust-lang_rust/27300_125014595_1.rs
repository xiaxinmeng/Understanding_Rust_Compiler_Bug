
<anon>:2:22: 5:7 error: the trait `core::ops::FnOnce<(_,)>` is not implemented for the type `core::option::Option<_>` [E0277]
<anon>:2     let p = Some(45).and_then({|x|
<anon>:3         println!("doubling {}", x);
<anon>:4         Some(100 * 2)
<anon>:5     });
<anon>:2:22: 5:7 help: see the detailed explanation for E0277
<anon>:4:9: 4:22 note: the expression at 4:9...
<anon>:4         Some(100 * 2)
                 ^~~~~~~~~~~~~
<anon>:2:32: 3:35 note: ...may have been intended to be part of closure expression at 2:32...
<anon>:2     let p = Some(45).and_then({|x|
<anon>:3         println!("doubling {}", x);
<anon>:3:34: 3:35 note: ...but closure expression ends at 3:34
<anon>:3         println!("doubling {}", x);
                                          ^
error: aborting due to previous error
