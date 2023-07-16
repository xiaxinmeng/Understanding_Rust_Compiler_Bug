plain
2019-10-27T19:45:25.8024663Z do so (now or later) by using -b with the checkout command again. Example:
2019-10-27T19:45:25.8024880Z 
2019-10-27T19:45:25.8024931Z   git checkout -b <new-branch-name>
2019-10-27T19:45:25.8024991Z 
2019-10-27T19:45:25.8025056Z HEAD is now at 6e3aa3034 Auto merge of #65878 - Centril:rollup-frxkcrv, r=Centril
2019-10-27T19:45:25.8420616Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-10-27T19:45:25.8534508Z ==============================================================================
2019-10-27T19:45:25.8534595Z Task         : Bash
2019-10-27T19:45:25.8534685Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-10-27T21:44:42.7711736Z ---- iter\traits\iterator.rs - iter::traits::iterator::Iterator::chain (line 431) stdout ----
2019-10-27T21:44:42.7711867Z error[E0412]: cannot find type `OsStr` in this scope
2019-10-27T21:44:42.7711952Z  --> iter\traits\iterator.rs:433:24
2019-10-27T21:44:42.7712051Z   |
2019-10-27T21:44:42.7712144Z 5 | fn os_str_to_utf16(s: &OsStr) -> Vec<u16> {
2019-10-27T21:44:42.7712337Z   |
2019-10-27T21:44:42.7712422Z help: possible candidate is found in another module, you can import it into scope
2019-10-27T21:44:42.7712527Z   |
2019-10-27T21:44:42.7712788Z 3 | use std::ffi::OsStr;
---
2019-10-27T21:44:42.7959262Z 
2019-10-27T21:44:42.7959321Z 
2019-10-27T21:44:42.8774172Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap test --exclude src/test/ui --exclude src/test/compile-fail
2019-10-27T21:44:42.8774940Z Build completed unsuccessfully in 1:49:21
2019-10-27T21:44:42.9164561Z make: *** [Makefile:89: ci-mingw-subset-1] Error 1
2019-10-27T21:44:42.9914168Z   local time: Sun Oct 27 21:44:42 CUT 2019
2019-10-27T21:44:43.5516482Z   network time: Sun, 27 Oct 2019 21:44:43 GMT
2019-10-27T21:44:43.5528170Z == end clock drift check ==
2019-10-27T21:44:43.6246566Z 
2019-10-27T21:44:43.6246566Z 
2019-10-27T21:44:43.9778634Z ##[error]Bash exited with code '2'.
2019-10-27T21:44:44.0572757Z ##[section]Starting: Upload CPU usage statistics
2019-10-27T21:44:44.1421153Z ==============================================================================
2019-10-27T21:44:44.1421295Z Task         : Bash
2019-10-27T21:44:44.1421381Z Description  : Run a Bash script on macOS, Linux, or Windows
