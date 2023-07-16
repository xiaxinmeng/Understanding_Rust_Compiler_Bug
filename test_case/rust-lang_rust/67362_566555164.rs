plain
2019-12-17T14:04:36.5622825Z     Checking rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2019-12-17T14:04:36.5623285Z error: this file contains an un-closed delimiter
2019-12-17T14:04:36.5623609Z    --> src/librustc_data_structures/sync.rs:794:3
2019-12-17T14:04:36.5623850Z     |
2019-12-17T14:04:36.5624137Z 29  | cfg_if! {
2019-12-17T14:04:36.5624673Z     |         - un-closed delimiter
2019-12-17T14:04:36.5625260Z 131 |                                     -> Result<T, T> {
2019-12-17T14:04:36.5625731Z     |                                                     - this delimiter might not be properly closed...
2019-12-17T14:04:36.5626017Z ...
2019-12-17T14:04:36.5626312Z 139 |             }
2019-12-17T14:04:36.5626312Z 139 |             }
2019-12-17T14:04:36.5626727Z     |             - ...as it matches this but it has different indentation
2019-12-17T14:04:36.5626993Z ...
2019-12-17T14:04:36.5627260Z 794 | }
2019-12-17T14:04:36.5627929Z     |   ^
2019-12-17T14:04:36.5628006Z 
2019-12-17T14:04:36.5628335Z error: expected one of `,`, `::`, `as`, or `}`, found `;`
2019-12-17T14:04:36.5628668Z    --> src/librustc_data_structures/sync.rs:320:71
2019-12-17T14:04:36.5629281Z 320 |         pub use std::sync::atomic::{AtomicBool, AtomicUsize, AtomicU32;
2019-12-17T14:04:36.5629281Z 320 |         pub use std::sync::atomic::{AtomicBool, AtomicUsize, AtomicU32;
2019-12-17T14:04:36.5629772Z     |                                                                       ^ expected one of `,`, `::`, `as`, or `}` here
2019-12-17T14:04:36.5629868Z 
2019-12-17T14:04:36.5630169Z error: expected one of `*`, `::`, `;`, `{`, or `}`, found `#`
2019-12-17T14:04:36.5630481Z    --> src/librustc_data_structures/sync.rs:321:9
2019-12-17T14:04:36.5631089Z 320 |         pub use std::sync::atomic::{AtomicBool, AtomicUsize, AtomicU32;
2019-12-17T14:04:36.5631089Z 320 |         pub use std::sync::atomic::{AtomicBool, AtomicUsize, AtomicU32;
2019-12-17T14:04:36.5631817Z     |                                                                        - expected one of `*`, `::`, `;`, `{`, or `}` here
2019-12-17T14:04:36.5632519Z 321 |         #[cfg(target_has_atomic = "64")]
2019-12-17T14:04:36.5632953Z 
2019-12-17T14:04:36.5633217Z error: aborting due to 3 previous errors
2019-12-17T14:04:36.5633290Z 
2019-12-17T14:04:36.5633562Z error: could not compile `rustc_data_structures`.
2019-12-17T14:04:36.5633562Z error: could not compile `rustc_data_structures`.
2019-12-17T14:04:36.5633883Z warning: build failed, waiting for other jobs to finish...
2019-12-17T14:04:47.7234611Z error: build failed
2019-12-17T14:04:47.7272277Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-12-17T14:04:47.7284544Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-12-17T14:04:47.7284769Z Build completed unsuccessfully in 0:03:36
2019-12-17T14:04:47.7371511Z == clock drift check ==
2019-12-17T14:04:47.7386782Z   local time: Tue Dec 17 14:04:47 UTC 2019
2019-12-17T14:04:47.7386782Z   local time: Tue Dec 17 14:04:47 UTC 2019
2019-12-17T14:04:48.0021306Z   network time: Tue, 17 Dec 2019 14:04:47 GMT
2019-12-17T14:04:48.0022162Z == end clock drift check ==
2019-12-17T14:04:48.8648292Z 
2019-12-17T14:04:48.8751316Z ##[error]Bash exited with code '1'.
2019-12-17T14:04:48.8785079Z ##[section]Starting: Checkout
2019-12-17T14:04:48.8787058Z ==============================================================================
2019-12-17T14:04:48.8787150Z Task         : Get sources
2019-12-17T14:04:48.8787248Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
