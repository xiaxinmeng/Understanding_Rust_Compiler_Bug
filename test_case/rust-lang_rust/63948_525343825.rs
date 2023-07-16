plain
2019-08-27T14:56:23.2462098Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-27T14:56:23.2671668Z ##[command]git config gc.auto 0
2019-08-27T14:56:23.2748195Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-27T14:56:23.2801636Z ##[command]git config --get-all http.proxy
2019-08-27T14:56:23.2935277Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63948/merge:refs/remotes/pull/63948/merge
---
2019-08-27T14:56:58.7152759Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-27T14:56:58.7152901Z 
2019-08-27T14:56:58.7154230Z   git checkout -b <new-branch-name>
2019-08-27T14:56:58.7154449Z 
2019-08-27T14:56:58.7154641Z HEAD is now at 18555b180 Merge c7559554453b2e0de85c8fc710392c7e634fabd7 into 7e0afdad28c4d1154176df6d35a14e738ec311af
2019-08-27T14:56:58.7314462Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-27T14:56:58.7317192Z ==============================================================================
2019-08-27T14:56:58.7317274Z Task         : Bash
2019-08-27T14:56:58.7317311Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-27T15:04:13.1501019Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2019-08-27T15:04:21.8883220Z     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-08-27T15:04:23.3693979Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-08-27T15:04:24.5870884Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-08-27T15:04:28.2870136Z error[E0609]: no field `link_ordinal` on type `&feature_gate::active::Features`
2019-08-27T15:04:28.2871214Z    --> src/libsyntax/feature_gate/builtin_attrs.rs:278:12
2019-08-27T15:04:28.2872065Z     |
2019-08-27T15:04:28.2873508Z 278 |     gated!(link_ordinal, Whitelisted, template!(Word), experimental!(link_ordinal)),
2019-08-27T15:04:28.2874186Z 
2019-08-27T15:04:32.3064606Z error: aborting due to previous error
2019-08-27T15:04:32.3065387Z 
2019-08-27T15:04:32.3066143Z For more information about this error, try `rustc --explain E0609`.
---
2019-08-27T15:04:32.3685604Z == clock drift check ==
2019-08-27T15:04:32.3710208Z   local time: Tue Aug 27 15:04:32 UTC 2019
2019-08-27T15:04:32.4639155Z   network time: Tue, 27 Aug 2019 15:04:32 GMT
2019-08-27T15:04:32.4639548Z == end clock drift check ==
2019-08-27T15:04:33.7973286Z ##[error]Bash exited with code '1'.
2019-08-27T15:04:33.8013816Z ##[section]Starting: Checkout
2019-08-27T15:04:33.8016278Z ==============================================================================
2019-08-27T15:04:33.8016663Z Task         : Get sources
2019-08-27T15:04:33.8016874Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
