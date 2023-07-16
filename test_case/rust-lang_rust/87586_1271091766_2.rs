` documentation tests, you *have* to use `cargo test --doc -- --ignored`. (Even `--all-targets` doesn't run documentation tests -- this surprised me, at least.)

It looks like this may have been a bugfix

cc @epage -- the only change in cargo's git history I saw in a quick skim that looked like it could've changed this behavior is the clap4 upgrade changing how `--` behaves; see [this comment for my suspicions](https://github.com/rust-lang/cargo/commit/69ba69f908e7400826a52d64d4361005e9e065a6#r86078435). I'm now fairly certain that clap4's changes to trailing varargs are the culprit here -- though this is I think mostly a beneficial change.