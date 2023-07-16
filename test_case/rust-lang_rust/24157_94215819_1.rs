
<anon>:6:5: 9:6 error: match arms have incompatible types:
 expected `bool`,
    found `()`
(expected bool,
    found ()) [E0308]
<anon>:6     match a {
<anon>:7         1 => true,
<anon>:8         _ => panic!("It's not one!"),
<anon>:9     }
<anon>:8:14: 8:38 note: match arm with an incompatible type
<anon>:8         _ => panic!("It's not one!"),
                      ^~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error
playpen: application terminated with error code 101
Program ended.
