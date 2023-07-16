ignore (cannot-test-this-because-xxxx)", if the annotation cannot be avoided.
2019-09-11T12:00:32.9439731Z 
2019-09-11T12:00:34.8391339Z some tidy checks failed
2019-09-11T12:00:34.8394452Z 
2019-09-11T12:00:34.8394540Z 
2019-09-11T12:00:34.8394540Z 
2019-09-11T12:00:34.8398175Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-11T12:00:34.8399675Z 
2019-09-11T12:00:34.8399901Z 
2019-09-11T12:00:34.8410925Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-11T12:00:34.8411228Z Build completed unsuccessfully in 0:01:58
2019-09-11T12:00:34.8411228Z Build completed unsuccessfully in 0:01:58
2019-09-11T12:00:34.8465183Z == clock drift check ==
2019-09-11T12:00:34.8493561Z   local time: Wed Sep 11 12:00:34 UTC 2019
2019-09-11T12:00:34.9352728Z   network time: Wed, 11 Sep 2019 12:00:34 GMT
2019-09-11T12:00:34.9354494Z == end clock drift check ==
2019-09-11T12:00:36.3461914Z ##[error]Bash exited with code '1'.
2019-09-11T12:00:36.3531955Z ##[section]Starting: Checkout
2019-09-11T12:00:36.3533809Z ==============================================================================
2019-09-11T12:00:36.3533959Z Task         : Get sources
2019-09-11T12:00:36.3534007Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
