
<anon>:3:28: 3:30 error: cannot move out of borrowed content
<anon>:3     let _x: &[[isize]] = &[*x, *x];
                                    ^~
<anon>:3:32: 3:34 error: cannot move out of borrowed content
<anon>:3     let _x: &[[isize]] = &[*x, *x];
                                        ^~
<anon>:3:28: 3:30 error: cannot move a value of type [isize]: the size of [isize] cannot be statically determined [E0161]
<anon>:3     let _x: &[[isize]] = &[*x, *x];
                                    ^~
<anon>:3:32: 3:34 error: cannot move a value of type [isize]: the size of [isize] cannot be statically determined [E0161]
<anon>:3     let _x: &[[isize]] = &[*x, *x];
                                        ^~
error: aborting due to 4 previous errors
playpen: application terminated with error code 101
