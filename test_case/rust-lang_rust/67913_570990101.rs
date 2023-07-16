plain
2020-01-06T03:24:03.1667517Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-06T03:24:03.1875482Z ##[command]git config gc.auto 0
2020-01-06T03:24:03.1943480Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-06T03:24:03.8139997Z ##[command]git config --get-all http.proxy
2020-01-06T03:24:03.8146279Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67913/merge:refs/remotes/pull/67913/merge
---
2020-01-06T03:28:54.1628439Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-06T03:28:54.1651572Z Checking std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-01-06T03:28:54.5665318Z    Compiling cc v1.0.47
2020-01-06T03:28:54.5665798Z     Checking core v0.0.0 (/checkout/src/libcore)
2020-01-06T03:28:58.4819600Z error: use of item 'f64::MIN_POSITIVE' that will be deprecated in future version 1.42.0: replaced by associated constant
2020-01-06T03:28:58.4820390Z   --> src/libcore/num/f64.rs:46:31
2020-01-06T03:28:58.4821693Z 46 | pub const MIN_POSITIVE: f64 = MIN_POSITIVE;
2020-01-06T03:28:58.4822016Z    |                               ^^^^^^^^^^^^
2020-01-06T03:28:58.4822299Z    |
2020-01-06T03:28:58.4822589Z    = note: `-D deprecated-in-future` implied by `-D warnings`
---
2020-01-06T03:29:06.1926803Z   local time: Mon Jan  6 03:29:06 UTC 2020
2020-01-06T03:29:06.4759611Z   network time: Mon, 06 Jan 2020 03:29:06 GMT
2020-01-06T03:29:06.4789330Z == end clock drift check ==
2020-01-06T03:29:22.3255543Z 
2020-01-06T03:29:22.3373155Z ##[error]Bash exited with code '1'.
2020-01-06T03:29:22.3403560Z ##[section]Starting: Checkout
2020-01-06T03:29:22.3405711Z ==============================================================================
2020-01-06T03:29:22.3405788Z Task         : Get sources
2020-01-06T03:29:22.3405838Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
