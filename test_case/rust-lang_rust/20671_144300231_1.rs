
<anon>:10:36: 10:42 error: the trait `for<'b, 'a> core::ops::Add<&'b V>` is not implemented for the type `&'a V` [E0277]
<anon>:10 fn test<S, V: Vector<S>>(v: V) { v.test() }
                                             ^~~~~~
<anon>:10:36: 10:42 help: see the detailed explanation for E0277
<anon>:10:36: 10:42 error: the trait `for<'a> core::ops::Mul<S>` is not implemented for the type `&'a V` [E0277]
<anon>:10 fn test<S, V: Vector<S>>(v: V) { v.test() }
                                             ^~~~~~
