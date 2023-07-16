ignore (cannot-test-this-because-xxxx)", if the annotation cannot be avoided.
2019-11-29T12:36:57.8414115Z 
2019-11-29T12:36:59.4687410Z Checking which error codes lack tests...
2019-11-29T12:36:59.6585350Z some tidy checks failed
2019-11-29T12:36:59.6585468Z Found 487 error codes
2019-11-29T12:36:59.6585468Z Found 487 error codes
2019-11-29T12:36:59.6585514Z Found 0 error codes with no tests
2019-11-29T12:36:59.6585559Z Done!
2019-11-29T12:36:59.6585607Z 
2019-11-29T12:36:59.6585635Z 
2019-11-29T12:36:59.6586484Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-29T12:36:59.6586611Z 
2019-11-29T12:36:59.6586639Z 
2019-11-29T12:36:59.6592395Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-29T12:36:59.6592488Z Build completed unsuccessfully in 0:01:23
2019-11-29T12:36:59.6592488Z Build completed unsuccessfully in 0:01:23
2019-11-29T12:36:59.6636206Z == clock drift check ==
2019-11-29T12:36:59.6644758Z   local time: Fri Nov 29 12:36:59 UTC 2019
2019-11-29T12:36:59.9381749Z   network time: Fri, 29 Nov 2019 12:36:59 GMT
2019-11-29T12:36:59.9383366Z == end clock drift check ==
2019-11-29T12:37:01.2896435Z 
2019-11-29T12:37:01.2999254Z ##[error]Bash exited with code '1'.
2019-11-29T12:37:01.3037455Z ##[section]Starting: Checkout
2019-11-29T12:37:01.3039108Z ==============================================================================
2019-11-29T12:37:01.3039166Z Task         : Get sources
2019-11-29T12:37:01.3039228Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
