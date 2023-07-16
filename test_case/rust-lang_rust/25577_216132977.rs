
error: main function not found
<anon>:2:5: 2:9 error: the trait bound `T: std::marker::Sized` is not satisfied [E0277]
<anon>:2     t: T,
             ^~~~
<anon>:2:5: 2:9 help: see the detailed explanation for E0277
<anon>:2:5: 2:9 help: consider adding a `where T: std::marker::Sized` bound
<anon>:2:5: 2:9 note: only the last field of a struct or enum variant may have a dynamically sized type
