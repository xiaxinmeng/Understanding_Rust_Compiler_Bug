plain
2019-08-20T15:19:49.9873241Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-20T15:19:50.0076607Z ##[command]git config gc.auto 0
2019-08-20T15:19:50.0116659Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-20T15:19:50.0181868Z ##[command]git config --get-all http.proxy
2019-08-20T15:19:50.0330688Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63745/merge:refs/remotes/pull/63745/merge
---
2019-08-20T15:20:25.1346214Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-20T15:20:25.1346242Z 
2019-08-20T15:20:25.1346451Z   git checkout -b <new-branch-name>
2019-08-20T15:20:25.1346479Z 
2019-08-20T15:20:25.1346523Z HEAD is now at d7dd4db7e Merge 5aa44a689b0c0bfb1790d70542fec3e98b26d44f into 51879c3abaedb926739095d19a2af638ee6a07d8
2019-08-20T15:20:25.1529356Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-20T15:20:25.1532255Z ==============================================================================
2019-08-20T15:20:25.1532309Z Task         : Bash
2019-08-20T15:20:25.1532355Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-20T15:25:57.9661785Z     Checking hashbrown v0.5.0
2019-08-20T15:25:58.9906429Z error: an inner attribute is not permitted following an outer doc comment
2019-08-20T15:25:58.9906973Z   --> src/libstd/os/linux/syscall/mod.rs:15:1
2019-08-20T15:25:58.9907213Z    |
2019-08-20T15:25:58.9907529Z 14 | /// Execute syscall with 0 arguments.
2019-08-20T15:25:58.9907892Z    | ------------------------------------- previous doc comment
2019-08-20T15:25:58.9908205Z 15 | #![unstable(feature = "linux_syscall", issue = "0")]
2019-08-20T15:25:58.9908659Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not permitted following an outer attibute
2019-08-20T15:25:58.9908908Z    |
2019-08-20T15:25:58.9909538Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2019-08-20T15:25:58.9922893Z error: an inner attribute is not permitted following an outer doc comment
2019-08-20T15:25:58.9923255Z   --> src/libstd/os/linux/syscall/mod.rs:22:1
2019-08-20T15:25:58.9923574Z    |
2019-08-20T15:25:58.9923932Z 21 | /// Execute syscall with 1 argument.
2019-08-20T15:25:58.9923932Z 21 | /// Execute syscall with 1 argument.
2019-08-20T15:25:58.9924277Z    | ------------------------------------ previous doc comment
2019-08-20T15:25:58.9924775Z 22 | #![unstable(feature = "linux_syscall", issue = "0")]
2019-08-20T15:25:58.9925217Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not permitted following an outer attibute
2019-08-20T15:25:58.9925503Z    |
2019-08-20T15:25:58.9926152Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2019-08-20T15:25:58.9941656Z error: an inner attribute is not permitted following an outer doc comment
2019-08-20T15:25:58.9941996Z   --> src/libstd/os/linux/syscall/mod.rs:29:1
2019-08-20T15:25:58.9942847Z    |
2019-08-20T15:25:58.9943172Z 28 | /// Execute syscall with 2 arguments.
2019-08-20T15:25:58.9943172Z 28 | /// Execute syscall with 2 arguments.
2019-08-20T15:25:58.9943479Z    | ------------------------------------- previous doc comment
2019-08-20T15:25:58.9944191Z 29 | #![unstable(feature = "linux_syscall", issue = "0")]
2019-08-20T15:25:58.9945047Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not permitted following an outer attibute
2019-08-20T15:25:58.9945303Z    |
2019-08-20T15:25:58.9945805Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2019-08-20T15:25:58.9946165Z error: an inner attribute is not permitted following an outer doc comment
2019-08-20T15:25:58.9946488Z   --> src/libstd/os/linux/syscall/mod.rs:36:1
2019-08-20T15:25:58.9946718Z    |
2019-08-20T15:25:58.9947008Z 35 | /// Execute syscall with 3 arguments.
2019-08-20T15:25:58.9947008Z 35 | /// Execute syscall with 3 arguments.
2019-08-20T15:25:58.9947394Z    | ------------------------------------- previous doc comment
2019-08-20T15:25:58.9947718Z 36 | #![unstable(feature = "linux_syscall", issue = "0")]
2019-08-20T15:25:58.9948294Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not permitted following an outer attibute
2019-08-20T15:25:58.9948537Z    |
2019-08-20T15:25:58.9948959Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2019-08-20T15:25:58.9959031Z error: an inner attribute is not permitted following an outer doc comment
2019-08-20T15:25:58.9959393Z   --> src/libstd/os/linux/syscall/mod.rs:43:1
2019-08-20T15:25:58.9959645Z    |
2019-08-20T15:25:58.9960092Z 42 | /// Execute syscall with 4 arguments.
2019-08-20T15:25:58.9960092Z 42 | /// Execute syscall with 4 arguments.
2019-08-20T15:25:58.9960582Z    | ------------------------------------- previous doc comment
2019-08-20T15:25:58.9990111Z 43 | #![unstable(feature = "linux_syscall", issue = "0")]
2019-08-20T15:25:58.9990526Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not permitted following an outer attibute
2019-08-20T15:25:58.9990778Z    |
2019-08-20T15:25:58.9991389Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2019-08-20T15:25:59.0032239Z error: an inner attribute is not permitted following an outer doc comment
2019-08-20T15:25:59.0032570Z   --> src/libstd/os/linux/syscall/mod.rs:52:1
2019-08-20T15:25:59.0032804Z    |
2019-08-20T15:25:59.0033517Z 51 | /// Execute syscall with 5 arguments.
2019-08-20T15:25:59.0033517Z 51 | /// Execute syscall with 5 arguments.
2019-08-20T15:25:59.0033858Z    | ------------------------------------- previous doc comment
2019-08-20T15:25:59.0034182Z 52 | #![unstable(feature = "linux_syscall", issue = "0")]
2019-08-20T15:25:59.0034932Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not permitted following an outer attibute
2019-08-20T15:25:59.0035348Z    |
2019-08-20T15:25:59.0035802Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2019-08-20T15:25:59.0036156Z error: an inner attribute is not permitted following an outer doc comment
2019-08-20T15:25:59.0036446Z   --> src/libstd/os/linux/syscall/mod.rs:61:1
2019-08-20T15:25:59.0036673Z    |
2019-08-20T15:25:59.0036960Z 60 | /// Execute syscall with 6 arguments.
2019-08-20T15:25:59.0036960Z 60 | /// Execute syscall with 6 arguments.
2019-08-20T15:25:59.0037306Z    | ------------------------------------- previous doc comment
2019-08-20T15:25:59.0037626Z 61 | #![unstable(feature = "linux_syscall", issue = "0")]
2019-08-20T15:25:59.0038007Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not permitted following an outer attibute
2019-08-20T15:25:59.0038410Z    |
2019-08-20T15:25:59.0038980Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2019-08-20T15:25:59.0039041Z 
2019-08-20T15:26:01.4472243Z error[E0061]: this function takes 6 parameters but 7 parameters were supplied
2019-08-20T15:26:01.4473297Z   --> src/libstd/os/linux/syscall/mod.rs:66:5
2019-08-20T15:26:01.4473848Z    |
2019-08-20T15:26:01.4475138Z 66 |       platform::syscall5(n, a1, a2, a3, a4, a5, a6)
2019-08-20T15:26:01.4476541Z    | 
2019-08-20T15:26:01.4476541Z    | 
2019-08-20T15:26:01.4477155Z   ::: src/libstd/os/linux/syscall/x86_64.rs:54:1
2019-08-20T15:26:01.4477681Z    |
2019-08-20T15:26:01.4478524Z 54 | / pub unsafe fn syscall5(n: usize, a1: usize, a2: usize, a3: usize,
2019-08-20T15:26:01.4479129Z 55 | |                                 a4: usize, a5: usize) -> usize {
2019-08-20T15:26:01.4479903Z 56 | |     let ret : usize;
2019-08-20T15:26:01.4480558Z 57 | |     asm!("syscall" : "={rax}"(ret)
2019-08-20T15:26:01.4482126Z 62 | |     ret
2019-08-20T15:26:01.4482693Z 63 | | }
2019-08-20T15:26:01.4483273Z    | |_- defined here
2019-08-20T15:26:01.4483536Z 
2019-08-20T15:26:01.4483536Z 
2019-08-20T15:26:01.7932348Z error: aborting due to 8 previous errors
2019-08-20T15:26:01.7932610Z 
2019-08-20T15:26:01.7933103Z For more information about this error, try `rustc --explain E0061`.
2019-08-20T15:26:01.8301099Z error: Could not compile `std`.
2019-08-20T15:26:01.8301207Z 
2019-08-20T15:26:01.8301482Z To learn more, run the command again with --verbose.
2019-08-20T15:26:01.8333695Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-20T15:26:01.8347038Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-20T15:26:01.8347127Z Build completed unsuccessfully in 0:02:45
2019-08-20T15:26:01.8400698Z == clock drift check ==
2019-08-20T15:26:01.8825655Z   local time: Tue Aug 20 15:26:01 UTC 2019
2019-08-20T15:26:01.8825655Z   local time: Tue Aug 20 15:26:01 UTC 2019
2019-08-20T15:26:02.0871058Z   network time: Tue, 20 Aug 2019 15:26:02 GMT
2019-08-20T15:26:02.0872024Z == end clock drift check ==
2019-08-20T15:26:09.1046132Z ##[error]Bash exited with code '1'.
2019-08-20T15:26:09.1107950Z ##[section]Starting: Checkout
2019-08-20T15:26:09.1109871Z ==============================================================================
2019-08-20T15:26:09.1109938Z Task         : Get sources
2019-08-20T15:26:09.1109984Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
