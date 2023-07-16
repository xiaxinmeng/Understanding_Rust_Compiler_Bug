 cmd
<anon>:2:20: 33:6 error: mismatched types:
 expected `usize`,
    found `()`
(expected usize,
    found ()) [E0308]
<anon>:2     let _: usize = while 4 > 3 {
<anon>:3         let _ = 3;
<anon>:4         let _ = 3;
<anon>:5         let _ = 3;
<anon>:6         let _ = 3;
<anon>:7         let _ = 3;
         ...
error: aborting due to previous error
playpen: application terminated with error code 101
