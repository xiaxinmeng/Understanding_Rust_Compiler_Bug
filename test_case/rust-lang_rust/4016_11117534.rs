
../src/test/run-pass/issue-4016.rs:12:29: 12:58 error: cannot infer an appropriate lifetime due to conflicting requirements
../src/test/run-pass/issue-4016.rs:12     let _v: T = deserialize(&json::Deserializer(move doc));
                                                                   ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~
note: first, the lifetime cannot outlive lifetime re_bound(br_self)...
../src/test/run-pass/issue-4016.rs:12:16: 12:27 note: ...due to the following expression
../src/test/run-pass/issue-4016.rs:12     let _v: T = deserialize(&json::Deserializer(move doc));
                                                      ^~~~~~~~~~~
../src/test/run-pass/issue-4016.rs:12:16: 12:59 note: but, the lifetime must be valid for the call at 12:16...
../src/test/run-pass/issue-4016.rs:12     let _v: T = deserialize(&json::Deserializer(move doc));
                                                      ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
../src/test/run-pass/issue-4016.rs:12:29: 12:58 note: ...due to the following expression
../src/test/run-pass/issue-4016.rs:12     let _v: T = deserialize(&json::Deserializer(move doc));
                                                                   ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~
error: aborting due to previous error
