
z.rs:6:14: 6:24 error: `MethodCall` is the name of a struct or struct variant, but this expression uses it like a function name [E0423]
z.rs:6     let mc = MethodCall.expr(); // ERROR: unresolved name `MethodCall` [E0425]
                    ^~~~~~~~~~
z.rs:6:14: 6:24 help: run `rustc --explain E0423` to see a detailed explanation
z.rs:6:14: 6:24 help: did you mean to write: `MethodCall { /* fields */ }`?
