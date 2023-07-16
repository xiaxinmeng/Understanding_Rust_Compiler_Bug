plain
2019-08-28T17:27:00.8972807Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-28T17:27:00.8972888Z 
2019-08-28T17:27:00.8973167Z   git checkout -b <new-branch-name>
2019-08-28T17:27:00.8973216Z 
2019-08-28T17:27:00.8973557Z HEAD is now at 391cdd3bd Auto merge of #63806 - mati865:rand, r=alexcrichton
2019-08-28T17:27:00.9139924Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-28T17:27:00.9143569Z ==============================================================================
2019-08-28T17:27:00.9143689Z Task         : Bash
2019-08-28T17:27:00.9143770Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-28T17:42:13.7799358Z tidy check
2019-08-28T17:42:14.6630958Z * 578 error codes
2019-08-28T17:42:14.6631151Z * highest error code: E0733
2019-08-28T17:42:15.0349128Z * 263 features
2019-08-28T17:42:15.4257543Z invalid license Apache-2.0 WITH LLVM-exception in /checkout/obj/build/tmp/distcheck/src/../vendor/wasi/Cargo.toml
2019-08-28T17:42:15.7661427Z some tidy checks failed
2019-08-28T17:42:15.7662684Z 
2019-08-28T17:42:15.7662684Z 
2019-08-28T17:42:15.7663988Z command did not execute successfully: "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/tmp/distcheck/src" "/checkout/obj/build/tmp/distcheck/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2019-08-28T17:42:15.7664236Z 
2019-08-28T17:42:15.7664272Z 
2019-08-28T17:42:15.7674569Z failed to run: /checkout/obj/build/tmp/distcheck/build/bootstrap/debug/bootstrap test
2019-08-28T17:42:15.7674757Z Build completed unsuccessfully in 0:03:23
2019-08-28T17:42:15.7674757Z Build completed unsuccessfully in 0:03:23
2019-08-28T17:42:15.7728504Z Makefile:48: recipe for target 'check' failed
2019-08-28T17:42:15.7728648Z 
2019-08-28T17:42:15.7728726Z command did not execute successfully: "make" "check"
2019-08-28T17:42:15.7728822Z expected success, got: exit code: 2
2019-08-28T17:42:15.7728868Z 
2019-08-28T17:42:15.7728868Z 
2019-08-28T17:42:15.7728905Z 
2019-08-28T17:42:15.7729005Z make: *** [check] Error 1
2019-08-28T17:42:15.7779619Z Build completed unsuccessfully in 0:10:30
2019-08-28T17:42:15.7784932Z == clock drift check ==
2019-08-28T17:42:15.7810885Z   local time: Wed Aug 28 17:42:15 UTC 2019
2019-08-28T17:42:15.8541012Z   network time: Wed, 28 Aug 2019 17:42:15 GMT
2019-08-28T17:42:15.8541012Z   network time: Wed, 28 Aug 2019 17:42:15 GMT
2019-08-28T17:42:15.8544290Z == end clock drift check ==
2019-08-28T17:42:17.7103026Z ##[error]Bash exited with code '1'.
2019-08-28T17:42:17.7145258Z ##[section]Starting: Upload CPU usage statistics
2019-08-28T17:42:17.7152383Z ==============================================================================
2019-08-28T17:42:17.7152484Z Task         : Bash
2019-08-28T17:42:17.7152578Z Description  : Run a Bash script on macOS, Linux, or Windows
