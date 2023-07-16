
<anon>:4:21: 4:26 error: the trait bound `T: std::marker::Reflect` is not satisfied [E0277]
<anon>:4     let value_any = value as &Any;
                             ^~~~~
<anon>:4:21: 4:26 help: see the detailed explanation for E0277
<anon>:4:21: 4:26 help: consider adding a `where T: std::marker::Reflect` bound
<anon>:4:21: 4:26 note: required for the cast to the object type `std::any::Any + 'static`
error: aborting due to previous error
