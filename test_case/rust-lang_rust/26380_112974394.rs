
<anon>:11:5: 11:18 error: cannot convert to a trait object because trait `Foo` is not object-safe [E0038]
<anon>:11     Box::new(Bar) // Line 11
              ^~~~~~~~~~~~~
<anon>:11:5: 11:18 note: method `foo` has no receiver
<anon>:11     Box::new(Bar) // Line 11
              ^~~~~~~~~~~~~
