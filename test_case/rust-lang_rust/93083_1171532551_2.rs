

which has the added benefit of `cargo test` reporting that the test was skipped if it wasn't run.

It works well enough by reusing the existing constructs that I would say there's no need to implement any new features if it weren't for the fact that this runs afoul of tools that operate on the source code as-is before any macro/attribute expansions or evaluations; see for example [how this approach breaks `rustfmt`](https://github.com/rust-lang/rustfmt/issues/5420) (while @Manishearth's approach B from above doesn't).