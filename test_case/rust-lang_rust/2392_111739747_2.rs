
<anon>:18:19: 18:22 error: type `Cat` does not implement any method in scope named `x`
<anon>:18     assert!(kitty.x() == 5);
                            ^~~
<anon>:18:19: 18:22 note: did you mean to write `kitty.x`?
<anon>:18     assert!(kitty.x() == 5);
                            ^~~
<anon>:19:19: 19:25 error: type `Cat` does not implement any method in scope named `func`
<anon>:19     let x = kitty.func();
                            ^~~~~~
<anon>:19:19: 19:25 note: use `(kitty.func)(...)` if you meant to call the function stored in the `func` field
<anon>:19     let x = kitty.func();
                            ^~~~~~
