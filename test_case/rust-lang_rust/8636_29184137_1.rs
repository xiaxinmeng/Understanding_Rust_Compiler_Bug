
<anon>:9:53: 9:62 error: cannot borrow `v[]` as mutable more than once at a time
<anon>:9         let mut v = [1, 2, 3]; match v { [ref mut x, ref mut y, ref mut z] => { *x += 1; *y += 1; *z += 1 } }
                                                              ^~~~~~~~~
<anon>:9:42: 9:51 note: second borrow of `v[]` as mutable occurs here
<anon>:9         let mut v = [1, 2, 3]; match v { [ref mut x, ref mut y, ref mut z] => { *x += 1; *y += 1; *z += 1 } }
                                                   ^~~~~~~~~
...

error: aborting due to 3 previous errors
application terminated with error code 101
