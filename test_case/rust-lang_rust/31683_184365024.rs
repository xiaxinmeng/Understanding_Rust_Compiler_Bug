
<anon>:1:1: 3:2 error: recursive type `T` has infinite size [E0072]
<anon>:1 struct T {
<anon>:2     t: T
<anon>:3 }
<anon>:1:1: 3:2 help: see the detailed explanation for E0072
<anon>:1:1: 3:2 note: field `t` of `T` has the same type as its container
<anon>:1:1: 3:2 help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to make `T` representable
