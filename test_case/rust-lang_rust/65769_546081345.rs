plain
2019-10-24T17:47:01.8280673Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-24T17:47:01.8497584Z ##[command]git config gc.auto 0
2019-10-24T17:47:01.8585716Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-24T17:47:01.8653839Z ##[command]git config --get-all http.proxy
2019-10-24T17:47:01.8816740Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65769/merge:refs/remotes/pull/65769/merge
---
2019-10-24T17:53:51.2492446Z    Compiling serde_json v1.0.40
2019-10-24T17:53:53.1167116Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-24T17:54:04.8524782Z     Finished release [optimized] target(s) in 1m 32s
2019-10-24T17:54:04.8613404Z tidy check
2019-10-24T17:54:06.1735850Z tidy error: duplicate error code: 741
2019-10-24T17:54:06.1737103Z tidy error: /checkout/src/librustc_resolve/error_codes.rs:1910: E0741: r##"
2019-10-24T17:54:06.1737520Z tidy error: /checkout/src/librustc_typeck/error_codes.rs:5008: E0741: r##"
2019-10-24T17:54:07.5201896Z Found 484 error codes
2019-10-24T17:54:07.5202044Z Found 0 error codes with no tests
2019-10-24T17:54:07.5202161Z Done!
2019-10-24T17:54:07.5202206Z some tidy checks failed
2019-10-24T17:54:07.5202206Z some tidy checks failed
2019-10-24T17:54:07.5202275Z 
2019-10-24T17:54:07.5203587Z 
2019-10-24T17:54:07.5204923Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-24T17:54:07.5205962Z 
2019-10-24T17:54:07.5206007Z 
2019-10-24T17:54:07.5220856Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-24T17:54:07.5220960Z Build completed unsuccessfully in 0:01:35
2019-10-24T17:54:07.5220960Z Build completed unsuccessfully in 0:01:35
2019-10-24T17:54:07.5270918Z == clock drift check ==
2019-10-24T17:54:07.5279195Z   local time: Thu Oct 24 17:54:07 UTC 2019
2019-10-24T17:54:07.6860660Z   network time: Thu, 24 Oct 2019 17:54:07 GMT
2019-10-24T17:54:07.6860807Z == end clock drift check ==
2019-10-24T17:54:09.1237031Z 
2019-10-24T17:54:09.1348628Z ##[error]Bash exited with code '1'.
2019-10-24T17:54:09.1383326Z ##[section]Starting: Checkout
2019-10-24T17:54:09.1384979Z ==============================================================================
2019-10-24T17:54:09.1385028Z Task         : Get sources
2019-10-24T17:54:09.1385066Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
