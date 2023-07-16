 text
<anon>:6:11: 2:27 error: non-scalar cast: `i32` as `()`
<anon>:6     cast!(2);
                   ^
<anon>:2     ($x:expr) => ($x as ())
                                 ^~
<anon>:1:1: 3:2 note: in expansion of cast!
<anon>:6:5: 6:14 note: expansion site
error: aborting due to previous error
