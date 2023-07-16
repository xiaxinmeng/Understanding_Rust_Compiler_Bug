
[00:39:53] error[E0631]: type mismatch in closure argument
[00:39:53]   --> $DIR/anonymous-higher-ranked-lifetime.rs:18:5
[00:39:53]    |
[00:39:53] 18 |     g2(|_: (), _: ()| {});
[00:39:53]    |            ^^ argument needs type `&()`
[00:39:53]    |     
[00:39:53]    = note: expected type `&()`
[00:39:53]               found type `()`
