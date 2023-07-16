plain
2019-12-17T13:57:06.4767563Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-17T13:57:06.5001204Z ##[command]git config gc.auto 0
2019-12-17T13:57:06.5068770Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-17T13:57:06.5136599Z ##[command]git config --get-all http.proxy
2019-12-17T13:57:06.5286931Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67362/merge:refs/remotes/pull/67362/merge
---
2019-12-17T14:03:50.2275475Z     Checking rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2019-12-17T14:03:50.2750879Z error: this file contains an un-closed delimiter
2019-12-17T14:03:50.2752296Z    --> src/librustc_data_structures/sync.rs:794:3
2019-12-17T14:03:50.2752738Z     |
2019-12-17T14:03:50.2753108Z 29  | cfg_if! {
2019-12-17T14:03:50.2753536Z     |         - un-closed delimiter
2019-12-17T14:03:50.2754422Z 131 |                                     -> Result<T, T> {
2019-12-17T14:03:50.2754912Z     |                                                     - this delimiter might not be properly closed...
2019-12-17T14:03:50.2755241Z ...
2019-12-17T14:03:50.2755622Z 139 |             }
2019-12-17T14:03:50.2755622Z 139 |             }
2019-12-17T14:03:50.2756247Z     |             - ...as it matches this but it has different indentation
2019-12-17T14:03:50.2756577Z ...
2019-12-17T14:03:50.2756949Z 794 | }
2019-12-17T14:03:50.2758202Z     |   ^
2019-12-17T14:03:50.2758416Z 
2019-12-17T14:03:50.3285898Z error: expected one of `,`, `::`, `as`, or `}`, found `;`
2019-12-17T14:03:50.3286550Z    --> src/librustc_data_structures/sync.rs:320:71
2019-12-17T14:03:50.3288056Z 320 |         pub use std::sync::atomic::{AtomicBool, AtomicUsize, AtomicU32;
2019-12-17T14:03:50.3288056Z 320 |         pub use std::sync::atomic::{AtomicBool, AtomicUsize, AtomicU32;
2019-12-17T14:03:50.3288676Z     |                                                                       ^ expected one of `,`, `::`, `as`, or `}` here
2019-12-17T14:03:50.3288891Z 
2019-12-17T14:03:50.3318452Z error: expected one of `*`, `::`, `;`, `{`, or `}`, found `#`
2019-12-17T14:03:50.3319085Z    --> src/librustc_data_structures/sync.rs:321:9
2019-12-17T14:03:50.3320033Z 320 |         pub use std::sync::atomic::{AtomicBool, AtomicUsize, AtomicU32;
2019-12-17T14:03:50.3320033Z 320 |         pub use std::sync::atomic::{AtomicBool, AtomicUsize, AtomicU32;
2019-12-17T14:03:50.3320960Z     |                                                                        - expected one of `*`, `::`, `;`, `{`, or `}` here
2019-12-17T14:03:50.3321551Z 321 |         #[cfg(target_has_atomic = "64")]
2019-12-17T14:03:50.3322234Z 
2019-12-17T14:03:50.3363447Z error: aborting due to 3 previous errors
2019-12-17T14:03:50.3363733Z 
2019-12-17T14:03:50.3404815Z error: could not compile `rustc_data_structures`.
2019-12-17T14:03:50.3404815Z error: could not compile `rustc_data_structures`.
2019-12-17T14:03:50.3415321Z warning: build failed, waiting for other jobs to finish...
2019-12-17T14:04:01.9724871Z error: build failed
2019-12-17T14:04:01.9744982Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-12-17T14:04:01.9758142Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-12-17T14:04:01.9758253Z Build completed unsuccessfully in 0:03:33
2019-12-17T14:04:01.9809117Z == clock drift check ==
2019-12-17T14:04:01.9824211Z   local time: Tue Dec 17 14:04:01 UTC 2019
2019-12-17T14:04:01.9824211Z   local time: Tue Dec 17 14:04:01 UTC 2019
2019-12-17T14:04:02.2673045Z   network time: Tue, 17 Dec 2019 14:04:02 GMT
2019-12-17T14:04:02.2678781Z == end clock drift check ==
2019-12-17T14:04:03.3160716Z 
2019-12-17T14:04:03.3261835Z ##[error]Bash exited with code '1'.
2019-12-17T14:04:03.3289958Z ##[section]Starting: Checkout
2019-12-17T14:04:03.3291441Z ==============================================================================
2019-12-17T14:04:03.3291491Z Task         : Get sources
2019-12-17T14:04:03.3291535Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
