
issue-31997.rs:30:19: 30:31 error: failed to resolve. Use of undeclared type or module `HashMap` [E0433]
issue-31997.rs:30     let mut map = HashMap::new();
                                    ^~~~~~~~~~~~
issue-31997.rs:30:19: 30:31 help: run `rustc --explain E0433` to see a detailed explanation
issue-31997.rs:30:19: 30:31 error: unresolved name `HashMap::new` [E0425]
issue-31997.rs:30     let mut map = HashMap::new();
                                    ^~~~~~~~~~~~
issue-31997.rs:30:19: 30:31 help: run `rustc --explain E0425` to see a detailed explanation
error: aborting due to 2 previous errors
