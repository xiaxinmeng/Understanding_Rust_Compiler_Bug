
foo.rs:16:19: 16:31 error: failed to resolve. Use of undeclared type or module `HashMap` [E0433]
foo.rs:16     let mut map = HashMap::new();
                            ^~~~~~~~~~~~
foo.rs:16:19: 16:31 help: run `rustc --explain E0433` to see a detailed explanation
foo.rs:16:19: 16:31 error: unresolved name `HashMap::new` [E0425]
foo.rs:16     let mut map = HashMap::new();
                            ^~~~~~~~~~~~
foo.rs:16:19: 16:31 help: run `rustc --explain E0425` to see a detailed explanation
foo.rs:33:25: 33:55 error: the type of this value must be known in this context
foo.rs:33         let mut entry = map.entry(instance.to_owned()).or_insert(Instance {
                                  ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
foo.rs:40:21: 40:30 error: the type of this value must be known in this context
foo.rs:40             assert!(entry.end.is_none());
                              ^~~~~~~~~
foo.rs:40:13: 40:42 note: in this expansion of assert! (defined in <std macros>)
error: aborting due to 2 previous errors
