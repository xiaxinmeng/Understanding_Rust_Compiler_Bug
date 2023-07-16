
<anon>:18:23: 22:6 error: `r` does not live long enough
<anon>:18     r.listen(Box::new(|| {
<anon>:19         x = Some(22);
<anon>:20         y = Some(44);
<anon>:21         r.something();
<anon>:22     }));
<anon>:15:22: 23:2 note: reference must be valid for the block suffix following statement 1 at 15:21...
<anon>:15     let mut y = None;
<anon>:16     let mut r = Reactor::new();
<anon>:17     
<anon>:18     r.listen(Box::new(|| {
<anon>:19         x = Some(22);
<anon>:20         y = Some(44);
          ...
<anon>:16:32: 23:2 note: ...but borrowed value is only valid for the block suffix following statement 2 at 16:31
<anon>:16     let mut r = Reactor::new();
<anon>:17     
<anon>:18     r.listen(Box::new(|| {
<anon>:19         x = Some(22);
<anon>:20         y = Some(44);
<anon>:21         r.something();
          ...
error: aborting due to previous error
playpen: application terminated with error code 101
