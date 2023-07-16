
2020-02-12T15:56:29.8209359Z     | ------- previous doc comment
2020-02-12T15:56:29.8209814Z 429 | #![feature(layout_for_ptr)]
2020-02-12T15:56:29.8210208Z     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ not permitted following an outer attibute
2020-02-12T15:56:29.8210457Z     |
2020-02-12T15:56:29.8211205Z     = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2020-02-12T15:56:39.2686597Z    Compiling libc v0.2.66
2020-02-12T15:56:40.0104652Z error: aborting due to 2 previous errors
2020-02-12T15:56:40.0104746Z 
2020-02-12T15:56:40.0272395Z error: could not compile `core`.
---
2020-02-12T15:56:40.2168856Z   local time: Wed Feb 12 15:56:40 UTC 2020
2020-02-12T15:56:40.3779843Z   network time: Wed, 12 Feb 2020 15:56:40 GMT
2020-02-12T15:56:40.3780081Z == end clock drift check ==
2020-02-12T15:56:48.2293291Z 
2020-02-12T15:56:48.2390157Z ##[error]Bash exited with code '1'.
2020-02-12T15:56:48.2403192Z ##[section]Finishing: Run build
2020-02-12T15:56:48.2419872Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69079/merge to s
2020-02-12T15:56:48.2421618Z Task         : Get sources
2020-02-12T15:56:48.2421682Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-12T15:56:48.2421728Z Version      : 1.0.0
2020-02-12T15:56:48.2421768Z Author       : Microsoft
2020-02-12T15:56:48.2421768Z Author       : Microsoft
2020-02-12T15:56:48.2421830Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-12T15:56:48.2421879Z ==============================================================================
2020-02-12T15:56:48.6547206Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-12T15:56:48.6588242Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69079/merge to s
2020-02-12T15:56:48.6702241Z Cleaning up task key
2020-02-12T15:56:48.6703243Z Start cleaning up orphan processes.
2020-02-12T15:56:48.7049404Z Terminate orphan process: pid (4474) (python)
2020-02-12T15:56:48.7070086Z ##[section]Finishing: Finalize Job
