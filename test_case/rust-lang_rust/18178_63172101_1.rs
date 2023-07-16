
<anon>:18:21: 18:30 error: the trait `Cancel` is not implemented for the type `C`
<anon>:18     let cancel_me = f.stuff();
                              ^~~~~~~~~
<anon>:19:15: 19:23 error: type `C` does not implement any method in scope named `cancel`
<anon>:19     cancel_me.cancel();
                        ^~~~~~~~
error: aborting due to 2 previous errors
playpen: application terminated with error code 101
