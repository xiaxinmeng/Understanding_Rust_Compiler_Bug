
<anon>:2:9: 2:10 error: mismatched types:
 expected `fn(&'b ()) -> &'a &'b ()`,
    found `fn(&'a ()) -> &'a &'b ()`
(expected concrete lifetime,
    found bound lifetime parameter 'a) [E0308]
<anon>:2     bar(f);
                 ^
<anon>:2:9: 2:10 help: see the detailed explanation for E0308
note: expected concrete lifetime is lifetime ReSkolemized(0, BrNamed(DefId { krate: 0, node: 31 }, 'b(64)))
error: aborting due to previous error
playpen: application terminated with error code 101
