
<anon>:6:19: 6:20 error: cannot infer an appropriate lifetime for automatic coercion due to conflicting requirements [E0495]
<anon>:6     foo(|s: &str| s); // not ok
                           ^
<anon>:4:1: 7:2 help: consider using an explicit lifetime parameter as shown: fn main()
<anon>:4 fn main() {
<anon>:5     foo(|s| s); // ok
<anon>:6     foo(|s: &str| s); // not ok
<anon>:7 }
