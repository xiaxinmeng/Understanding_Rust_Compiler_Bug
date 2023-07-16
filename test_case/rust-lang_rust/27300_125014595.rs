
<anon>:4:14: 4:15 error: unresolved name `x` [E0425]
<anon>:4         Some(x * 2)
                      ^
<anon>:2:33: 2:34 note: nearest candidate defined at 2:33...
<anon>:2    let p = Some(45).and_then({|x|
                                        ^
<anon>:3:35: 3:36 note: ...but goes out of scope at 3:35
<anon>:3        println!("doubling {}", x);
                                          ^
error: aborting due to previous error
