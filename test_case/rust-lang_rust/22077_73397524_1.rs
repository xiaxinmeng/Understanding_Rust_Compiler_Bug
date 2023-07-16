
<anon>:8:5: 10:6 error: method `call` has an incompatible type for trait: expected bound lifetime parameter 'x, found concrete lifetime [E0053]
<anon>:8     fn call<'b>(&'b self) -> &'b str {
<anon>:9         &self.x[]
<anon>:10     }
