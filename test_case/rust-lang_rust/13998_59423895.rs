 rust
<anon>:24:22: 24:30 error: cannot infer an appropriate lifetime due to conflicting requirements
<anon>:24     let f = Bar::new(two_args);
                               ^~~~~~~~
note: first, the lifetime cannot outlive lifetime ReInfer(ReSkolemized(0u, BrAnon(0u)))...
<anon>:24:22: 24:30 note: ...so that expression is assignable (expected `fn(&int)`, found `fn(&int)`)
<anon>:24     let f = Bar::new(two_args);
                               ^~~~~~~~
