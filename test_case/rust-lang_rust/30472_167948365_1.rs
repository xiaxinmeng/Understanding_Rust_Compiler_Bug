
<anon>:23:1: 25:2 error: type mismatch resolving `for<'self_> <usize as A>::S == <usize as B<'self_>>::T`:
 expected usize,
    found associated type [E0271]
<anon>:23 impl C for usize {
<anon>:24     type U = usize;
<anon>:25 }
<anon>:23:1: 25:2 help: see the detailed explanation for E0271
<anon>:23:1: 25:2 note: required by `C`
error: aborting due to previous error
playpen: application terminated with error code 101
