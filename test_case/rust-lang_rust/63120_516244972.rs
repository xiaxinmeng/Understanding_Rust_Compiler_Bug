plain
2019-07-30T01:11:17.0554428Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-30T01:11:17.0554476Z 
2019-07-30T01:11:17.0554678Z   git checkout -b <new-branch-name>
2019-07-30T01:11:17.0554734Z 
2019-07-30T01:11:17.0554984Z HEAD is now at 6d72deb1b Auto merge of #63120 - Centril:rollup-swgmy2h, r=Centril
2019-07-30T01:11:17.0694785Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-30T01:11:17.0698608Z ==============================================================================
2019-07-30T01:11:17.0698722Z Task         : Bash
2019-07-30T01:11:17.0698794Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-30T03:23:05.0139276Z 
2019-07-30T03:23:05.0139311Z 
2019-07-30T03:23:05.0147099Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/test/run-fail/pretty src/test/run-pass-valgrind/pretty src/tools/cargo src/tools/cargotest
2019-07-30T03:23:05.0147245Z Build completed unsuccessfully in 2:07:05
2019-07-30T03:23:05.0201221Z make: *** [check-aux] Error 1
2019-07-30T03:23:05.0202161Z Makefile:50: recipe for target 'check-aux' failed
2019-07-30T03:23:08.6430071Z ##[error]Bash exited with code '2'.
2019-07-30T03:23:08.6469137Z ##[section]Starting: Upload CPU usage statistics
2019-07-30T03:23:08.6476621Z ==============================================================================
2019-07-30T03:23:08.6476834Z Task         : Bash
2019-07-30T03:23:08.6476911Z Description  : Run a Bash script on macOS, Linux, or Windows
