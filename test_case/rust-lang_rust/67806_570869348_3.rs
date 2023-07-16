ignore (cannot-test-this-because-xxxx)", if the annotation cannot be avoided.
2020-01-05T05:42:05.0763387Z 
2020-01-05T05:42:06.8921357Z Checking which error codes lack tests...
2020-01-05T05:42:07.1131451Z some tidy checks failed
2020-01-05T05:42:07.1131640Z Found 486 error codes
2020-01-05T05:42:07.1131640Z Found 486 error codes
2020-01-05T05:42:07.1131688Z Found 0 error codes with no tests
2020-01-05T05:42:07.1131772Z Done!
2020-01-05T05:42:07.1137379Z 
2020-01-05T05:42:07.1137453Z 
2020-01-05T05:42:07.1139038Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-01-05T05:42:07.1139187Z 
2020-01-05T05:42:07.1139210Z 
2020-01-05T05:42:07.1154622Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-01-05T05:42:07.1154713Z Build completed unsuccessfully in 0:01:34
2020-01-05T05:42:07.1154713Z Build completed unsuccessfully in 0:01:34
2020-01-05T05:42:07.1209779Z == clock drift check ==
2020-01-05T05:42:07.1235825Z   local time: Sun Jan  5 05:42:07 UTC 2020
2020-01-05T05:42:07.2622395Z   network time: Sun, 05 Jan 2020 05:42:07 GMT
2020-01-05T05:42:07.2622495Z == end clock drift check ==
2020-01-05T05:42:08.6862062Z 
2020-01-05T05:42:08.6946619Z ##[error]Bash exited with code '1'.
2020-01-05T05:42:08.6975120Z ##[section]Starting: Checkout
2020-01-05T05:42:08.6976777Z ==============================================================================
2020-01-05T05:42:08.6977492Z Task         : Get sources
2020-01-05T05:42:08.6977564Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
