plain
2020-01-05T14:33:12.5450769Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-05T14:33:12.5696389Z ##[command]git config gc.auto 0
2020-01-05T14:33:12.5766109Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-05T14:33:12.5820095Z ##[command]git config --get-all http.proxy
2020-01-05T14:33:12.5959879Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67871/merge:refs/remotes/pull/67871/merge
---
2020-01-05T14:42:17.3432025Z     Checking rustc-rayon v0.3.0
2020-01-05T14:42:26.6566102Z    Compiling serde_derive v1.0.81
2020-01-05T14:42:49.6886125Z     Checking serde_json v1.0.40
2020-01-05T14:42:50.8992511Z     Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
2020-01-05T14:42:53.0866262Z error[E0599]: no method named `as_deref` found for type `std::option::Option<clean::types::Stability>` in the current scope
2020-01-05T14:42:53.0867108Z     --> src/librustdoc/html/render.rs:4072:56
2020-01-05T14:42:53.0867607Z      |
2020-01-05T14:42:53.0868154Z 4072 |                     if let Some(stab) = item.stability.as_deref().filter(|stab| stab.level == stability::Unstable) {
2020-01-05T14:42:53.0869421Z      |
2020-01-05T14:42:53.0869902Z      = note: the method `as_deref` exists but the following trait bounds were not satisfied:
2020-01-05T14:42:53.0869902Z      = note: the method `as_deref` exists but the following trait bounds were not satisfied:
2020-01-05T14:42:53.0870350Z              `clean::types::Stability : std::ops::Deref`
2020-01-05T14:42:53.4533492Z error: aborting due to previous error
2020-01-05T14:42:53.4534171Z 
2020-01-05T14:42:53.4534656Z For more information about this error, try `rustc --explain E0599`.
2020-01-05T14:42:53.4657437Z error: could not compile `rustdoc`.
---
2020-01-05T14:42:53.4765738Z   local time: Sun Jan  5 14:42:53 UTC 2020
2020-01-05T14:42:53.7422412Z   network time: Sun, 05 Jan 2020 14:42:53 GMT
2020-01-05T14:42:53.7425282Z == end clock drift check ==
2020-01-05T14:42:55.1983338Z 
2020-01-05T14:42:55.2085642Z ##[error]Bash exited with code '1'.
2020-01-05T14:42:55.2118796Z ##[section]Starting: Checkout
2020-01-05T14:42:55.2120603Z ==============================================================================
2020-01-05T14:42:55.2120657Z Task         : Get sources
2020-01-05T14:42:55.2120701Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
