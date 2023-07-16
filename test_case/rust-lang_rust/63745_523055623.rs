plain
2019-08-20T14:53:11.1580193Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-20T14:53:11.1760355Z ##[command]git config gc.auto 0
2019-08-20T14:53:11.1833162Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-20T14:53:11.1900739Z ##[command]git config --get-all http.proxy
2019-08-20T14:53:11.2031576Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63745/merge:refs/remotes/pull/63745/merge
---
2019-08-20T14:53:47.6991137Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-20T14:53:47.6991166Z 
2019-08-20T14:53:47.6991354Z   git checkout -b <new-branch-name>
2019-08-20T14:53:47.6991380Z 
2019-08-20T14:53:47.6991442Z HEAD is now at d10a5cbe2 Merge d4cb657dd4a967d55bde2ec5edcd7ad8e2bef868 into 14890954ce17c44d944eda988c5a64bb4c5ec9eb
2019-08-20T14:53:47.7146383Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-20T14:53:47.7149699Z ==============================================================================
2019-08-20T14:53:47.7149759Z Task         : Bash
2019-08-20T14:53:47.7149806Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-20T14:59:12.9210211Z     Checking hashbrown v0.5.0
2019-08-20T14:59:13.9283103Z error: an inner attribute is not permitted following an outer doc comment
2019-08-20T14:59:13.9283664Z   --> src/libstd/os/linux/syscall/mod.rs:15:1
2019-08-20T14:59:13.9284226Z    |
2019-08-20T14:59:13.9284483Z 14 | /// Execute syscall with 0 arguments.
2019-08-20T14:59:13.9284923Z    | ------------------------------------- previous doc comment
2019-08-20T14:59:13.9285230Z 15 | #![unstable(feature = "linux_syscall", issue = "0")]
2019-08-20T14:59:13.9285565Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not permitted following an outer attibute
2019-08-20T14:59:13.9286254Z    |
2019-08-20T14:59:13.9286723Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2019-08-20T14:59:13.9294520Z error: an inner attribute is not permitted following an outer doc comment
2019-08-20T14:59:13.9294808Z   --> src/libstd/os/linux/syscall/mod.rs:22:1
2019-08-20T14:59:13.9294998Z    |
2019-08-20T14:59:13.9295231Z 21 | /// Execute syscall with 1 argument.
2019-08-20T14:59:13.9295231Z 21 | /// Execute syscall with 1 argument.
2019-08-20T14:59:13.9296197Z    | ------------------------------------ previous doc comment
2019-08-20T14:59:13.9296543Z 22 | #![unstable(feature = "linux_syscall", issue = "0")]
2019-08-20T14:59:13.9297132Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not permitted following an outer attibute
2019-08-20T14:59:13.9297422Z    |
2019-08-20T14:59:13.9297852Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2019-08-20T14:59:13.9309841Z error: an inner attribute is not permitted following an outer doc comment
2019-08-20T14:59:13.9310335Z   --> src/libstd/os/linux/syscall/mod.rs:29:1
2019-08-20T14:59:13.9310783Z    |
2019-08-20T14:59:13.9311050Z 28 | /// Execute syscall with 2 arguments.
2019-08-20T14:59:13.9311050Z 28 | /// Execute syscall with 2 arguments.
2019-08-20T14:59:13.9311377Z    | ------------------------------------- previous doc comment
2019-08-20T14:59:13.9311677Z 29 | #![unstable(feature = "linux_syscall", issue = "0")]
2019-08-20T14:59:13.9312193Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not permitted following an outer attibute
2019-08-20T14:59:13.9312400Z    |
2019-08-20T14:59:13.9313494Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2019-08-20T14:59:13.9325326Z error: an inner attribute is not permitted following an outer doc comment
2019-08-20T14:59:13.9326404Z   --> src/libstd/os/linux/syscall/mod.rs:36:1
2019-08-20T14:59:13.9326700Z    |
2019-08-20T14:59:13.9326993Z 35 | /// Execute syscall with 3 arguments.
2019-08-20T14:59:13.9326993Z 35 | /// Execute syscall with 3 arguments.
2019-08-20T14:59:13.9327342Z    | ------------------------------------- previous doc comment
2019-08-20T14:59:13.9327655Z 36 | #![unstable(feature = "linux_syscall", issue = "0")]
2019-08-20T14:59:13.9328020Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not permitted following an outer attibute
2019-08-20T14:59:13.9328280Z    |
2019-08-20T14:59:13.9328709Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2019-08-20T14:59:13.9337652Z error: an inner attribute is not permitted following an outer doc comment
2019-08-20T14:59:13.9337966Z   --> src/libstd/os/linux/syscall/mod.rs:43:1
2019-08-20T14:59:13.9338226Z    |
2019-08-20T14:59:13.9338513Z 42 | /// Execute syscall with 4 arguments.
2019-08-20T14:59:13.9338513Z 42 | /// Execute syscall with 4 arguments.
2019-08-20T14:59:13.9338981Z    | ------------------------------------- previous doc comment
2019-08-20T14:59:13.9339285Z 43 | #![unstable(feature = "linux_syscall", issue = "0")]
2019-08-20T14:59:13.9339593Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not permitted following an outer attibute
2019-08-20T14:59:13.9339937Z    |
2019-08-20T14:59:13.9340350Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2019-08-20T14:59:13.9348798Z error: an inner attribute is not permitted following an outer doc comment
2019-08-20T14:59:13.9349256Z   --> src/libstd/os/linux/syscall/mod.rs:52:1
2019-08-20T14:59:13.9349672Z    |
2019-08-20T14:59:13.9349914Z 51 | /// Execute syscall with 5 arguments.
2019-08-20T14:59:13.9349914Z 51 | /// Execute syscall with 5 arguments.
2019-08-20T14:59:13.9350186Z    | ------------------------------------- previous doc comment
2019-08-20T14:59:13.9350469Z 52 | #![unstable(feature = "linux_syscall", issue = "0")]
2019-08-20T14:59:13.9350771Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not permitted following an outer attibute
2019-08-20T14:59:13.9350957Z    |
2019-08-20T14:59:13.9351322Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2019-08-20T14:59:13.9357385Z error: an inner attribute is not permitted following an outer doc comment
2019-08-20T14:59:13.9357678Z   --> src/libstd/os/linux/syscall/mod.rs:61:1
2019-08-20T14:59:13.9357908Z    |
2019-08-20T14:59:13.9358225Z 60 | /// Execute syscall with 6 arguments.
2019-08-20T14:59:13.9358225Z 60 | /// Execute syscall with 6 arguments.
2019-08-20T14:59:13.9358564Z    | ------------------------------------- previous doc comment
2019-08-20T14:59:13.9359209Z 61 | #![unstable(feature = "linux_syscall", issue = "0")]
2019-08-20T14:59:13.9359531Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not permitted following an outer attibute
2019-08-20T14:59:13.9359728Z    |
2019-08-20T14:59:13.9360096Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2019-08-20T14:59:13.9360141Z 
2019-08-20T14:59:16.3201712Z error[E0061]: this function takes 6 parameters but 7 parameters were supplied
2019-08-20T14:59:16.3202692Z   --> src/libstd/os/linux/syscall/mod.rs:66:5
2019-08-20T14:59:16.3203158Z    |
2019-08-20T14:59:16.3203590Z 66 |       platform::syscall5(n, a1, a2, a3, a4, a5, a6)
2019-08-20T14:59:16.3204449Z    | 
2019-08-20T14:59:16.3204449Z    | 
2019-08-20T14:59:16.3205230Z   ::: src/libstd/os/linux/syscall/x86_64.rs:55:1
2019-08-20T14:59:16.3206017Z    |
2019-08-20T14:59:16.3206733Z 55 | / pub unsafe fn syscall5(n: usize, a1: usize, a2: usize, a3: usize,
2019-08-20T14:59:16.3207320Z 56 | |                                 a4: usize, a5: usize) -> usize {
2019-08-20T14:59:16.3207825Z 57 | |     let ret : usize;
2019-08-20T14:59:16.3208301Z 58 | |     asm!("syscall" : "={rax}"(ret)
2019-08-20T14:59:16.3209479Z 63 | |     ret
2019-08-20T14:59:16.3209896Z 64 | | }
2019-08-20T14:59:16.3210305Z    | |_- defined here
2019-08-20T14:59:16.3210453Z 
2019-08-20T14:59:16.3210453Z 
2019-08-20T14:59:16.6785444Z error: aborting due to 8 previous errors
2019-08-20T14:59:16.6785726Z 
2019-08-20T14:59:16.6786095Z For more information about this error, try `rustc --explain E0061`.
2019-08-20T14:59:16.7137807Z error: Could not compile `std`.
2019-08-20T14:59:16.7137915Z 
2019-08-20T14:59:16.7138235Z To learn more, run the command again with --verbose.
2019-08-20T14:59:16.7172463Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-20T14:59:16.7179151Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-20T14:59:16.7179373Z Build completed unsuccessfully in 0:02:40
2019-08-20T14:59:16.7237330Z == clock drift check ==
2019-08-20T14:59:16.7253568Z   local time: Tue Aug 20 14:59:16 UTC 2019
2019-08-20T14:59:16.7253568Z   local time: Tue Aug 20 14:59:16 UTC 2019
2019-08-20T14:59:16.8161226Z   network time: Tue, 20 Aug 2019 14:59:16 GMT
2019-08-20T14:59:16.8165050Z == end clock drift check ==
2019-08-20T14:59:23.4489510Z ##[error]Bash exited with code '1'.
2019-08-20T14:59:23.4523874Z ##[section]Starting: Checkout
2019-08-20T14:59:23.4527543Z ==============================================================================
2019-08-20T14:59:23.4527606Z Task         : Get sources
2019-08-20T14:59:23.4527673Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
