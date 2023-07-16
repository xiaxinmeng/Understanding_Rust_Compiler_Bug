 rust
<anon>:22:5: 24:6 error: method `deref` has an incompatible type for trait:
 expected bound lifetime parameter ,
    found concrete lifetime [E0053]
<anon>:22     fn deref(&self) -> &DoesStuff {
<anon>:23         self as &DoesStuff
<anon>:24     }
<anon>:22:5: 24:6 help: see the detailed explanation for E0053
error: aborting due to previous error
playpen: application terminated with error code 101
