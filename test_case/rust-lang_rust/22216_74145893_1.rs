 rust
<anon>:4:5: 6:6 error: a `while` loop expression of type `()` is being returned but `usize` is expected
<anon>:4:5: 6:6 error: mismatched types:
 expected `usize`,
    found `()`
(expected usize,
    found ()) [E0308]
<anon>:4     while 4 > 3 {
<anon>:5         /* do something */
<anon>:6     }
error: aborting due to previous error
