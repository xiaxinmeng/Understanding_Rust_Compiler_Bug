
<anon>:3:35: 3:36 warning: value assigned to `a` is never read, #[warn(unused_assignments)] on by default
<anon>:3     let b = (a, a = 6, a, a * (a, a = 7).0);
                                           ^
(5, (), 6, 42)
