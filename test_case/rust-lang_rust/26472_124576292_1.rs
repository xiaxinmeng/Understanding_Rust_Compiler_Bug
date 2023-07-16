
<anon>:52:17: 52:20 error: no method named `x` found for type `inner::Dog` in the current scope
<anon>:52     let _ = dog.x();
                          ^~~
<anon>:52:17: 52:20 note: did you mean to write `dog.x`?
<anon>:52     let _ = dog.x();
                          ^~~
<anon>:53:17: 53:20 error: no method named `c` found for type `inner::Dog` in the current scope
<anon>:53     let _ = dog.c();
                          ^~~
<anon>:53:17: 53:20 note: use `(dog.c)(...)` if you meant to call the function stored in the `c` field
<anon>:53     let _ = dog.c();
                          ^~~
<anon>:55:17: 55:20 error: no method named `d` found for type `inner::Dog` in the current scope
<anon>:55     let _ = dog.d();
                          ^~~
<anon>:55:17: 55:20 note: use `(dog.d)(...)` if you meant to call the function stored in the `d` field
<anon>:55     let _ = dog.d();
                          ^~~
<anon>:55:17: 55:20 note: found defined static methods, maybe a `self` is missing?
<anon>:18:9: 20:10 note: candidate #1 is defined in an impl for the type `inner::Dog`
<anon>:18         pub fn d() -> usize {
<anon>:19             3
<anon>:20         }
error: aborting due to 3 previous errors
playpen: application terminated with error code 101
