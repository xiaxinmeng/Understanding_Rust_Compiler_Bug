
> 
> This must not compile, because bar is not const and therefore can't be evaluated at compile time. Btw, it raises(?) "can't call non const fn inside of const one", and can be considered incorrect downcasting. (set of valid const fns is smaller than set of fns at all)

Of course this must not compile, but I don't think that's related to what @filtsin was talking about.