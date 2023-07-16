plain
2019-07-24T00:17:23.1013803Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-24T00:17:23.1014069Z 
2019-07-24T00:17:23.1014523Z   git checkout -b <new-branch-name>
2019-07-24T00:17:23.1014770Z 
2019-07-24T00:17:23.1015269Z HEAD is now at f6e4b181d Auto merge of #62262 - varkor:must_use-adt-components-ii, r=<try>
2019-07-24T00:17:23.1150685Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-24T00:17:23.1153920Z ==============================================================================
2019-07-24T00:17:23.1154015Z Task         : Bash
2019-07-24T00:17:23.1154257Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-24T02:33:50.9047624Z [RUSTC-TIMING] git2 test:false 16.468
2019-07-24T02:33:50.9059792Z    Compiling git2-curl v0.10.0
2019-07-24T02:33:52.3503824Z [RUSTC-TIMING] git2_curl test:false 1.440
2019-07-24T02:33:52.3521255Z    Compiling cargo v0.39.0 (/checkout/src/tools/cargo)
2019-07-24T02:34:06.1822658Z error: unused boxed boxed `std::ops::FnMut` trait object in field `socket` that must be used
2019-07-24T02:34:06.1828414Z   --> src/tools/cargo/src/cargo/ops/cargo_package.rs:47:9
2019-07-24T02:34:06.1828806Z    |
2019-07-24T02:34:06.1829126Z 47 |         ops::resolve_ws(ws)?;
2019-07-24T02:34:06.1829725Z    |
2019-07-24T02:34:06.1829725Z    |
2019-07-24T02:34:06.1830050Z    = note: `#[deny(unused_must_use)]` on by default
2019-07-24T02:34:06.1830612Z    = note: closures are lazy and do nothing unless called
2019-07-24T02:34:06.1830693Z 
2019-07-24T02:34:06.1831225Z error: unused boxed boxed `std::ops::FnMut` trait object in field `timer` that must be used
2019-07-24T02:34:06.1831577Z   --> src/tools/cargo/src/cargo/ops/cargo_package.rs:47:9
2019-07-24T02:34:06.1831846Z    |
2019-07-24T02:34:06.1832165Z 47 |         ops::resolve_ws(ws)?;
2019-07-24T02:34:06.1832955Z    |
2019-07-24T02:34:06.1832955Z    |
2019-07-24T02:34:06.1833602Z    = note: closures are lazy and do nothing unless called
2019-07-24T02:34:06.3628725Z error: aborting due to 2 previous errors
2019-07-24T02:34:06.3629507Z 
2019-07-24T02:34:06.4762557Z [RUSTC-TIMING] cargo test:false 14.119
2019-07-24T02:34:06.4829570Z error: Could not compile `cargo`.
2019-07-24T02:34:06.4829570Z error: Could not compile `cargo`.
2019-07-24T02:34:06.4830110Z 
2019-07-24T02:34:06.4830606Z To learn more, run the command again with --verbose.
2019-07-24T02:34:06.4857621Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/cargo/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--message-format" "json"
2019-07-24T02:34:06.4858371Z expected success, got: exit code: 101
2019-07-24T02:34:06.4870224Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
2019-07-24T02:34:06.4870571Z Build completed unsuccessfully in 2:09:33
2019-07-24T02:34:07.4696400Z ##[error]Bash exited with code '1'.
2019-07-24T02:34:07.4731440Z ##[section]Starting: Upload CPU usage statistics
2019-07-24T02:34:07.4740283Z ==============================================================================
2019-07-24T02:34:07.4740381Z Task         : Bash
2019-07-24T02:34:07.4740641Z Description  : Run a Bash script on macOS, Linux, or Windows
