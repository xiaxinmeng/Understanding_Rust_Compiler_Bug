plain
2019-08-21T05:53:54.1411654Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-21T05:53:54.1602875Z ##[command]git config gc.auto 0
2019-08-21T05:53:54.7620984Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-21T05:53:54.7629403Z ##[command]git config --get-all http.proxy
2019-08-21T05:53:54.7635590Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/60966/merge:refs/remotes/pull/60966/merge
---
2019-08-21T05:54:28.8721456Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-21T05:54:28.8723068Z 
2019-08-21T05:54:28.8724612Z   git checkout -b <new-branch-name>
2019-08-21T05:54:28.8726328Z 
2019-08-21T05:54:28.8726965Z HEAD is now at 057305268 Merge 43ebe165e5935b37e6042652a7625fc8d44ab557 into bea0372a1a7a31b81f28cc4d9a83a2dc9a79d008
2019-08-21T05:54:28.8860767Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-21T05:54:28.8863636Z ==============================================================================
2019-08-21T05:54:28.8863714Z Task         : Bash
2019-08-21T05:54:28.8863761Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-21T06:27:49.2243277Z    Compiling arena v0.0.0 (/checkout/src/libarena)
2019-08-21T06:27:56.0621783Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-08-21T06:28:04.1565364Z    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2019-08-21T06:28:22.8023808Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-08-21T06:28:40.4071689Z error[E0658]: diagnostic items compiler internal support for linting
2019-08-21T06:28:40.4072996Z    --> src/librustc/ty/context.rs:981:28
2019-08-21T06:28:40.4073482Z     |
2019-08-21T06:28:40.4074000Z 981 | #[cfg_attr(not(bootstrap), rustc_diagnostic_item = "TyCtxt")]
2019-08-21T06:28:40.4075032Z     |
2019-08-21T06:28:40.4075032Z     |
2019-08-21T06:28:40.4075566Z     = help: add `#![feature(rustc_diagnostic_items)]` to the crate attributes to enable
2019-08-21T06:28:40.4075792Z 
2019-08-21T06:28:40.4123864Z error[E0658]: diagnostic items compiler internal support for linting
2019-08-21T06:28:40.4124540Z   --> src/librustc/ty/sty.rs:89:28
2019-08-21T06:28:40.4125002Z    |
2019-08-21T06:28:40.4125541Z 89 | #[cfg_attr(not(bootstrap), rustc_diagnostic_item = "TyKind")]
2019-08-21T06:28:40.4126667Z    |
2019-08-21T06:28:40.4126667Z    |
2019-08-21T06:28:40.4127279Z    = help: add `#![feature(rustc_diagnostic_items)]` to the crate attributes to enable
2019-08-21T06:28:40.4127509Z 
2019-08-21T06:28:40.4215615Z error[E0658]: diagnostic items compiler internal support for linting
2019-08-21T06:28:41.3643365Z     |
2019-08-21T06:28:41.3643365Z     |
2019-08-21T06:28:41.3643867Z 583 | #[cfg_attr(not(bootstrap), rustc_diagnostic_item = "Ty")]
2019-08-21T06:28:41.3644685Z     |
2019-08-21T06:28:41.3644685Z     |
2019-08-21T06:28:41.3645141Z     = help: add `#![feature(rustc_diagnostic_items)]` to the crate attributes to enable
2019-08-21T06:29:03.1161561Z error: aborting due to 3 previous errors
2019-08-21T06:29:03.1163643Z 
2019-08-21T06:29:03.1174298Z For more information about this error, try `rustc --explain E0658`.
2019-08-21T06:29:03.4189317Z error: Could not compile `rustc`.
---
2019-08-21T06:30:02.3431282Z == clock drift check ==
2019-08-21T06:30:02.3451122Z   local time: Wed Aug 21 06:30:02 UTC 2019
2019-08-21T06:30:02.4957826Z   network time: Wed, 21 Aug 2019 06:30:02 GMT
2019-08-21T06:30:02.4962882Z == end clock drift check ==
2019-08-21T06:30:03.5780954Z ##[error]Bash exited with code '1'.
2019-08-21T06:30:03.5814748Z ##[section]Starting: Checkout
2019-08-21T06:30:03.5817374Z ==============================================================================
2019-08-21T06:30:03.5817423Z Task         : Get sources
2019-08-21T06:30:03.5817462Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
