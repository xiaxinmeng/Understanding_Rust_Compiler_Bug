plain
2020-01-27T15:21:06.1609567Z    Compiling toml v0.5.3
2020-01-27T15:21:06.4660586Z    Compiling serde_json v1.0.40
2020-01-27T15:21:09.1520957Z    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
2020-01-27T15:21:38.1735723Z     Finished dev [unoptimized] target(s) in 1m 11s
2020-01-27T15:21:38.3935291Z thread 'main' panicked at 'bootstrapping from a dev compiler in a stable release, but should only be bootstrapping from a released compiler!', src/bootstrap/sanity.rs:243:13
2020-01-27T15:21:38.3945376Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
2020-01-27T15:21:38.3946090Z Build completed unsuccessfully in 0:01:21
2020-01-27T15:21:38.3992074Z make: *** [prepare] Error 1
2020-01-27T15:21:39.4016732Z Command failed. Attempt 2/5:
2020-01-27T15:21:39.4016732Z Command failed. Attempt 2/5:
2020-01-27T15:21:39.6313546Z     Finished dev [unoptimized] target(s) in 0.17s
2020-01-27T15:21:39.6783246Z thread 'main' panicked at 'bootstrapping from a dev compiler in a stable release, but should only be bootstrapping from a released compiler!', src/bootstrap/sanity.rs:243:13
2020-01-27T15:21:39.6788918Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
2020-01-27T15:21:39.6789187Z Build completed unsuccessfully in 0:00:00
2020-01-27T15:21:39.6826831Z make: *** [prepare] Error 1
2020-01-27T15:21:41.6846173Z Command failed. Attempt 3/5:
2020-01-27T15:21:41.6846173Z Command failed. Attempt 3/5:
2020-01-27T15:21:41.9160415Z     Finished dev [unoptimized] target(s) in 0.16s
2020-01-27T15:21:41.9746876Z thread 'main' panicked at 'bootstrapping from a dev compiler in a stable release, but should only be bootstrapping from a released compiler!', src/bootstrap/sanity.rs:243:13
2020-01-27T15:21:41.9757220Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
2020-01-27T15:21:41.9757343Z Build completed unsuccessfully in 0:00:00
2020-01-27T15:21:41.9803630Z make: *** [prepare] Error 1
2020-01-27T15:21:44.9820837Z Command failed. Attempt 4/5:
2020-01-27T15:21:44.9820837Z Command failed. Attempt 4/5:
2020-01-27T15:21:45.1797390Z     Finished dev [unoptimized] target(s) in 0.14s
2020-01-27T15:21:45.2414323Z thread 'main' panicked at 'bootstrapping from a dev compiler in a stable release, but should only be bootstrapping from a released compiler!', src/bootstrap/sanity.rs:243:13
2020-01-27T15:21:45.2422281Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
2020-01-27T15:21:45.2423290Z Build completed unsuccessfully in 0:00:00
2020-01-27T15:21:45.2461364Z make: *** [prepare] Error 1
2020-01-27T15:21:49.2479539Z Command failed. Attempt 5/5:
2020-01-27T15:21:49.2479539Z Command failed. Attempt 5/5:
2020-01-27T15:21:49.4546303Z     Finished dev [unoptimized] target(s) in 0.15s
2020-01-27T15:21:49.5154751Z thread 'main' panicked at 'bootstrapping from a dev compiler in a stable release, but should only be bootstrapping from a released compiler!', src/bootstrap/sanity.rs:243:13
2020-01-27T15:21:49.5163677Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build nonexistent/path/to/trigger/cargo/metadata
2020-01-27T15:21:49.5164087Z Build completed unsuccessfully in 0:00:00
2020-01-27T15:21:49.5200634Z make: *** [prepare] Error 1
2020-01-27T15:21:49.5204092Z The command has failed after 5 attempts.
2020-01-27T15:21:49.5204092Z The command has failed after 5 attempts.
2020-01-27T15:21:49.5206644Z == clock drift check ==
2020-01-27T15:21:49.5218732Z   local time: Mon Jan 27 15:21:49 UTC 2020
2020-01-27T15:21:49.9720010Z   network time: Mon, 27 Jan 2020 15:21:49 GMT
2020-01-27T15:21:49.9722693Z == end clock drift check ==
2020-01-27T15:21:55.2054245Z 
2020-01-27T15:21:55.2141448Z ##[error]Bash exited with code '1'.
2020-01-27T15:21:55.2171170Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-01-27T15:21:55.2172842Z ==============================================================================
2020-01-27T15:21:55.2173028Z Task         : Get sources
2020-01-27T15:21:55.2173110Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
