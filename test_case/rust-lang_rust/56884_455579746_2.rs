

This looks like [one of Cargo's tests](https://github.com/rust-lang/cargo/blob/2b4a5f1f0bb6e13759e88ea9512527b0beba154f/tests/testsuite/doc.rs#L653-L685), where it's scanning the output of rustdoc to make sure Cargo hasn't swallowed it. However, i'm not totally sure what the error is saying here.

@rust-lang/cargo, this PR changes the output format of doctest failures, and it seems to have tripped up one of Cargo's tests. How should we change the Cargo test to make it work? (And how should we coordinate the submodule update with this PR? `>_>`)