plain
2019-09-27T02:08:14.9959339Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-27T02:08:15.0148573Z ##[command]git config gc.auto 0
2019-09-27T02:08:15.0234092Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-27T02:08:15.0302818Z ##[command]git config --get-all http.proxy
2019-09-27T02:08:15.0471373Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64699/merge:refs/remotes/pull/64699/merge
---
2019-09-27T02:14:20.8155667Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-27T02:14:20.8180995Z Checking std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-09-27T02:14:21.2528141Z    Compiling cc v1.0.35
2019-09-27T02:14:21.2529774Z     Checking core v0.0.0 (/checkout/src/libcore)
2019-09-27T02:14:21.6503144Z error: couldn't read src/libcore/../stdarch/crates/core_arch/src/../stdarch/crates/core_arch/src/core_arch_docs.md: No such file or directory (os error 2)
2019-09-27T02:14:21.6503575Z   --> src/libcore/../stdarch/crates/core_arch/src/mod.rs:13:19
2019-09-27T02:14:21.6503930Z    |
2019-09-27T02:14:21.6504241Z 13 |     doc(include = "../stdarch/crates/core_arch/src/core_arch_docs.md")
2019-09-27T02:14:21.6504666Z    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ couldn't read file
2019-09-27T02:14:29.6369735Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-09-27T02:14:31.2803421Z    Compiling libc v0.2.62
2019-09-27T02:14:32.2202466Z    Compiling std v0.0.0 (/checkout/src/libstd)
2019-09-27T02:14:32.7354626Z error: aborting due to previous error
---
2019-09-27T02:14:34.4993168Z == clock drift check ==
2019-09-27T02:14:34.5006936Z   local time: Fri Sep 27 02:14:34 UTC 2019
2019-09-27T02:14:34.6525012Z   network time: Fri, 27 Sep 2019 02:14:34 GMT
2019-09-27T02:14:34.6528447Z == end clock drift check ==
2019-09-27T02:14:49.2370724Z ##[error]Bash exited with code '1'.
2019-09-27T02:14:49.2416376Z ##[section]Starting: Checkout
2019-09-27T02:14:49.2418526Z ==============================================================================
2019-09-27T02:14:49.2418608Z Task         : Get sources
2019-09-27T02:14:49.2418661Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
