ignore (cannot-test-this-because-xxxx)", if the annotation cannot be avoided.
2020-01-05T12:12:57.6931695Z 
2020-01-05T12:12:59.5282544Z Checking which error codes lack tests...
2020-01-05T12:12:59.7484661Z some tidy checks failed
2020-01-05T12:12:59.7484874Z Found 486 error codes
2020-01-05T12:12:59.7484874Z Found 486 error codes
2020-01-05T12:12:59.7484925Z Found 0 error codes with no tests
2020-01-05T12:12:59.7485014Z Done!
2020-01-05T12:12:59.7486233Z 
2020-01-05T12:12:59.7486261Z 
2020-01-05T12:12:59.7487237Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-01-05T12:12:59.7487386Z 
2020-01-05T12:12:59.7487412Z 
2020-01-05T12:12:59.7499132Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-01-05T12:12:59.7499244Z Build completed unsuccessfully in 0:01:40
2020-01-05T12:12:59.7499244Z Build completed unsuccessfully in 0:01:40
2020-01-05T12:12:59.7550040Z == clock drift check ==
2020-01-05T12:12:59.7562227Z   local time: Sun Jan  5 12:12:59 UTC 2020
2020-01-05T12:12:59.9220647Z   network time: Sun, 05 Jan 2020 12:12:59 GMT
2020-01-05T12:12:59.9224183Z == end clock drift check ==
2020-01-05T12:13:01.2988953Z 
2020-01-05T12:13:01.3108981Z ##[error]Bash exited with code '1'.
2020-01-05T12:13:01.3143997Z ##[section]Starting: Checkout
2020-01-05T12:13:01.3145963Z ==============================================================================
2020-01-05T12:13:01.3146021Z Task         : Get sources
2020-01-05T12:13:01.3146070Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
