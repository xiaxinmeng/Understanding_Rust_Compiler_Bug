plain
2019-08-23T20:48:10.0676191Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-23T20:48:10.0888411Z ##[command]git config gc.auto 0
2019-08-23T20:48:10.0965879Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-23T20:48:10.1026181Z ##[command]git config --get-all http.proxy
2019-08-23T20:48:10.1175758Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63834/merge:refs/remotes/pull/63834/merge
---
2019-08-23T20:48:45.3003285Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-23T20:48:45.3003318Z 
2019-08-23T20:48:45.3003565Z   git checkout -b <new-branch-name>
2019-08-23T20:48:45.3003598Z 
2019-08-23T20:48:45.3003651Z HEAD is now at 0acedf7d7 Merge 22893938381a65b15f645386b0b8ea0d669b26c5 into 9eae1fc0ea9b00341b8fe191582c4decb5cb86a3
2019-08-23T20:48:45.3193557Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-23T20:48:45.3197066Z ==============================================================================
2019-08-23T20:48:45.3197150Z Task         : Bash
2019-08-23T20:48:45.3197201Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-23T20:52:57.5756527Z    Compiling serde_derive v1.0.81
2019-08-23T20:53:15.8834018Z    Compiling serde_json v1.0.40
2019-08-23T20:53:16.2665272Z    Compiling toml v0.5.3
2019-08-23T20:53:19.4579216Z    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
2019-08-23T20:53:21.1862723Z error[E0277]: `std::path::Path` doesn't implement `std::fmt::Display`
2019-08-23T20:53:21.1903345Z     --> src/bootstrap/test.rs:1138:44
2019-08-23T20:53:21.1903428Z      |
2019-08-23T20:53:21.1903710Z 1138 |             cmd.arg(format!("-Clinker={}", linker));
2019-08-23T20:53:21.1909509Z      |                                            ^^^^^^ `std::path::Path` cannot be formatted with the default formatter; call `.display()` on it
2019-08-23T20:53:21.1909819Z      |
2019-08-23T20:53:21.1909976Z      = help: the trait `std::fmt::Display` is not implemented for `std::path::Path`
2019-08-23T20:53:21.1910496Z      = note: call `.display()` or `.to_string_lossy()` to safely print paths, as they may contain non-Unicode data
2019-08-23T20:53:21.1910715Z      = note: required because of the requirements on the impl of `std::fmt::Display` for `&std::path::Path`
2019-08-23T20:53:21.1910871Z      = note: required by `std::fmt::Display::fmt`
2019-08-23T20:53:22.6836097Z error: aborting due to previous error
2019-08-23T20:53:22.6836212Z 
2019-08-23T20:53:22.6837156Z For more information about this error, try `rustc --explain E0277`.
2019-08-23T20:53:22.7501747Z error: Could not compile `bootstrap`.
2019-08-23T20:53:22.7501747Z error: Could not compile `bootstrap`.
2019-08-23T20:53:22.7501844Z 
2019-08-23T20:53:22.7503156Z To learn more, run the command again with --verbose.
2019-08-23T20:53:22.7547042Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
2019-08-23T20:53:22.7599226Z == clock drift check ==
2019-08-23T20:53:22.7611091Z   local time: Fri Aug 23 20:53:22 UTC 2019
2019-08-23T20:53:22.8333311Z   network time: Fri, 23 Aug 2019 20:53:22 GMT
2019-08-23T20:53:22.8337659Z == end clock drift check ==
2019-08-23T20:53:22.8337659Z == end clock drift check ==
2019-08-23T20:53:26.1826231Z ##[error]Bash exited with code '1'.
2019-08-23T20:53:26.1860496Z ##[section]Starting: Checkout
2019-08-23T20:53:26.1862342Z ==============================================================================
2019-08-23T20:53:26.1862403Z Task         : Get sources
2019-08-23T20:53:26.1862460Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
