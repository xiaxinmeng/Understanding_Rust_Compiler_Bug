plain
2020-02-11T13:52:48.9759970Z #                                                                          2.0%
2020-02-11T13:52:48.9760493Z ######################################################################## 100.0%
2020-02-11T13:52:49.2810653Z extracting /checkout/obj/build/cache/2020-01-31/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz
2020-02-11T13:52:49.4183576Z     Updating crates.io index
2020-02-11T13:53:04.6538732Z error: failed to select a version for the requirement `backtrace = "^0.3.44"`
2020-02-11T13:53:04.6539872Z   candidate versions found which didn't match: 0.3.43, 0.3.41, 0.3.40, ...
2020-02-11T13:53:04.6540141Z   location searched: crates.io index
2020-02-11T13:53:04.6547065Z required by package `std v0.0.0 (/checkout/src/libstd)`
2020-02-11T13:53:04.6547867Z Build completed unsuccessfully in 0:00:29
2020-02-11T13:53:04.6548160Z make: *** [prepare] Error 1
2020-02-11T13:53:05.1414794Z Command failed. Attempt 2/5:
2020-02-11T13:53:05.2821668Z     Updating crates.io index
2020-02-11T13:53:05.2821668Z     Updating crates.io index
2020-02-11T13:53:05.5238703Z error: failed to select a version for the requirement `backtrace = "^0.3.44"`
2020-02-11T13:53:05.5239819Z   candidate versions found which didn't match: 0.3.43, 0.3.41, 0.3.40, ...
2020-02-11T13:53:05.5239930Z   location searched: crates.io index
2020-02-11T13:53:05.5240002Z required by package `std v0.0.0 (/checkout/src/libstd)`
2020-02-11T13:53:05.5257312Z Build completed unsuccessfully in 0:00:00
2020-02-11T13:53:05.5300513Z make: *** [prepare] Error 1
2020-02-11T13:53:07.5318219Z Command failed. Attempt 3/5:
2020-02-11T13:53:07.6614729Z     Updating crates.io index
2020-02-11T13:53:07.6614729Z     Updating crates.io index
2020-02-11T13:53:07.8606305Z error: failed to select a version for the requirement `backtrace = "^0.3.44"`
2020-02-11T13:53:07.8607737Z   candidate versions found which didn't match: 0.3.43, 0.3.41, 0.3.40, ...
2020-02-11T13:53:07.8608028Z   location searched: crates.io index
2020-02-11T13:53:07.8608157Z required by package `std v0.0.0 (/checkout/src/libstd)`
2020-02-11T13:53:07.8626884Z Build completed unsuccessfully in 0:00:00
2020-02-11T13:53:07.8670380Z make: *** [prepare] Error 1
2020-02-11T13:53:10.8691898Z Command failed. Attempt 4/5:
2020-02-11T13:53:11.0086180Z     Updating crates.io index
2020-02-11T13:53:11.0086180Z     Updating crates.io index
2020-02-11T13:53:11.2482372Z error: failed to select a version for the requirement `backtrace = "^0.3.44"`
2020-02-11T13:53:11.2483492Z   candidate versions found which didn't match: 0.3.43, 0.3.41, 0.3.40, ...
2020-02-11T13:53:11.2483612Z   location searched: crates.io index
2020-02-11T13:53:11.2483684Z required by package `std v0.0.0 (/checkout/src/libstd)`
2020-02-11T13:53:11.2507174Z Build completed unsuccessfully in 0:00:00
2020-02-11T13:53:11.2549065Z make: *** [prepare] Error 1
2020-02-11T13:53:15.2563674Z Command failed. Attempt 5/5:
2020-02-11T13:53:15.4012415Z     Updating crates.io index
2020-02-11T13:53:15.4012415Z     Updating crates.io index
2020-02-11T13:53:15.6245583Z error: failed to select a version for the requirement `backtrace = "^0.3.44"`
2020-02-11T13:53:15.6246539Z   candidate versions found which didn't match: 0.3.43, 0.3.41, 0.3.40, ...
2020-02-11T13:53:15.6246651Z   location searched: crates.io index
2020-02-11T13:53:15.6246748Z required by package `std v0.0.0 (/checkout/src/libstd)`
2020-02-11T13:53:15.6264840Z Build completed unsuccessfully in 0:00:00
2020-02-11T13:53:15.6311245Z make: *** [prepare] Error 1
2020-02-11T13:53:15.6313818Z The command has failed after 5 attempts.
2020-02-11T13:53:15.6316892Z == clock drift check ==
2020-02-11T13:53:15.6316892Z == clock drift check ==
2020-02-11T13:53:15.6331195Z   local time: Tue Feb 11 13:53:15 UTC 2020
2020-02-11T13:53:15.9049420Z   network time: Tue, 11 Feb 2020 13:53:15 GMT
2020-02-11T13:53:15.9055318Z == end clock drift check ==
2020-02-11T13:53:22.7830534Z 
2020-02-11T13:53:22.7925467Z ##[error]Bash exited with code '1'.
2020-02-11T13:53:22.7966033Z ##[section]Starting: Checkout rust-lang/rust@try to s
2020-02-11T13:53:22.7968234Z ==============================================================================
2020-02-11T13:53:22.7968326Z Task         : Get sources
2020-02-11T13:53:22.7968427Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
