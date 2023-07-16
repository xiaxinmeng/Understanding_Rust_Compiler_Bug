 bash
$ export DYLD_LIBRARY_PATH=/usr/local/lib/rustlib/x86_64-apple-darwin/lib


touch target/snapshot/bin/cargo
"/usr/local/bin/rustc" -v
rustc 0.12.0-dev
target/snapshot/bin/cargo build --target x86_64-apple-darwin  
   Compiling toml v0.1.0 (https://github.com/alexcrichton/toml-rs#05c1eb42)
   Compiling semver v0.1.0 (https://github.com/rust-lang/semver#5dee9181)
   Compiling openssl-static-sys v0.0.1 (https://github.com/alexcrichton/openssl-static-sys#b0180139)
   Compiling curl-sys v0.0.1 (https://github.com/alexcrichton/curl-rust?ref=bundle#1d43e08f)
   Compiling flate2 v0.0.1 (https://github.com/alexcrichton/flate2-rs#68971ae7)
   Compiling encoding v0.1.0 (https://github.com/lifthrasiir/rust-encoding#ff55916f)
   Compiling glob v0.0.1 (https://github.com/rust-lang/glob#27338cbd)
   Compiling link-config v0.0.1 (https://github.com/alexcrichton/link-config#0afbf50b)
   Compiling tar v0.0.1 (https://github.com/alexcrichton/tar-rs#943d7c01)
   Compiling libssh2-static-sys v0.0.1 (https://github.com/alexcrichton/libssh2-static-sys#80e71a30)
   Compiling docopt v0.6.4 (https://github.com/docopt/docopt.rs#21ce1190)
   Compiling url v0.1.0 (https://github.com/servo/rust-url#55b18b79)
   Compiling curl v0.0.1 (https://github.com/alexcrichton/curl-rust?ref=bundle#1d43e08f)
   Compiling docopt_macros v0.6.4 (https://github.com/docopt/docopt.rs#21ce1190)
   Compiling libgit2 v0.0.1 (https://github.com/alexcrichton/git2-rs#c01b0b27)
Could not compile `libgit2`.

--- stderr
/Users/amitava/.cargo/git/checkouts/git2-rs-cf258e7bcbaee34d/master/libgit2/src/lib.rs:79:9: 79:14 error: no rules expected the token `const`
/Users/amitava/.cargo/git/checkouts/git2-rs-cf258e7bcbaee34d/master/libgit2/src/lib.rs:79         const GIT_REVPARSE_SINGLE = 1 << 0,

