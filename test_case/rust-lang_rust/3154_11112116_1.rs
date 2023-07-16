
test/run-pass/issue-3154.rs:6:4: 6:9 error: cannot infer an appropriate lifetime due to conflicting requirements
test/run-pass/issue-3154.rs:6     thing{ x: x }
                                  ^~~~~
test/run-pass/issue-3154.rs:5:31: 7:1 note: first, the lifetime cannot outlive the anonymous lifetime #1 defined on the block at 5:31...
test/run-pass/issue-3154.rs:5 fn thing<Q>(x: &Q) -> thing<Q> {
test/run-pass/issue-3154.rs:6     thing{ x: x }
test/run-pass/issue-3154.rs:7 }
test/run-pass/issue-3154.rs:6:14: 6:15 note: ...due to the following expression
test/run-pass/issue-3154.rs:6     thing{ x: x }
                                            ^
test/run-pass/issue-3154.rs:5:31: 7:1 note: but, the lifetime must be valid for the anonymous lifetime #2 defined on the block at 5:31...
test/run-pass/issue-3154.rs:5 fn thing<Q>(x: &Q) -> thing<Q> {
test/run-pass/issue-3154.rs:6     thing{ x: x }
test/run-pass/issue-3154.rs:7 }
test/run-pass/issue-3154.rs:6:4: 6:9 note: ...due to the following expression
test/run-pass/issue-3154.rs:6     thing{ x: x }
                                  ^~~~~
