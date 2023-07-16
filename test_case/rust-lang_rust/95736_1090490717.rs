console
$ cd ~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/
$ cargo metadata --locked
warning: please specify `--format-version` flag explicitly to avoid compatibility problems
    Updating crates.io index
error: the lock file ~/.rustup/toolchains/stable-x86_64-apple-darwin/lib/rustlib/src/rust/library/std/Cargo.lock needs to be updated but --locked was passed to prevent this
If you want to try to generate the lock file without accessing the network, remove the --locked flag and use --offline instead.
