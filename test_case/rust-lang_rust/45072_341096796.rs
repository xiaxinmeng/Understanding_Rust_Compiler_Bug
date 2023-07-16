
[00:39:53] error[E0631]: type mismatch in closure arguments
[00:39:53]   --> $DIR/anonymous-higher-ranked-lifetime.rs:18:5
[00:39:53]    |
[00:39:53] 18 |     g2(|_: (), _: ()| {});
[00:39:53]    |     ^^ ----------------- found signature of `fn((), ()) -> _`
[00:39:53]    |     |
[00:39:53]    |     expected signature of `for<'r> fn(&'r (), for<'s> fn(&'s ())) -> _`
[00:39:53]    |
[00:39:53]    = note: required by `g2`
