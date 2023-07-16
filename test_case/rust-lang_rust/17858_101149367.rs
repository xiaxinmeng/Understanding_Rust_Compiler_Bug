
E0141.rs:10:16: 10:17 error: borrowed value does not live long enough
E0141.rs:10     let _ = S(&T);
                           ^
E0141.rs:10:5: 10:19 note: reference must be valid for the destruction scope surrounding statement at 10:4...
E0141.rs:10     let _ = S(&T);
                ^~~~~~~~~~~~~~
E0141.rs:10:5: 10:19 note: ...but borrowed value is only valid for the statement at 10:4
E0141.rs:10     let _ = S(&T);
                ^~~~~~~~~~~~~~
E0141.rs:10:5: 10:19 help: consider using a `let` binding to increase its lifetime
E0141.rs:10     let _ = S(&T);
                ^~~~~~~~~~~~~~
error: aborting due to previous error
