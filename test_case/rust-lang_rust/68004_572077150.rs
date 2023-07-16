plain
2020-01-08T14:03:20.7419889Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-08T14:03:20.7508444Z ##[command]git config gc.auto 0
2020-01-08T14:03:20.7596598Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-08T14:03:20.7649570Z ##[command]git config --get-all http.proxy
2020-01-08T14:03:20.7809094Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68004/merge:refs/remotes/pull/68004/merge
---
2020-01-08T14:17:39.4618507Z    Compiling serde_json v1.0.40
2020-01-08T14:17:41.1285790Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-01-08T14:17:51.0477387Z     Finished release [optimized] target(s) in 1m 22s
2020-01-08T14:17:51.0588204Z tidy check
2020-01-08T14:17:51.3581595Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0746.md: too many trailing newlines (3)
2020-01-08T14:17:53.9275051Z Found 488 error codes
2020-01-08T14:17:53.9275375Z Found 1 error codes with no tests
2020-01-08T14:17:53.9275609Z Done!
2020-01-08T14:17:53.9275657Z Error code E0192 needs to have at least one UI test!
2020-01-08T14:17:53.9275657Z Error code E0192 needs to have at least one UI test!
2020-01-08T14:17:53.9275724Z some tidy checks failed
2020-01-08T14:17:53.9275791Z 
2020-01-08T14:17:53.9275818Z 
2020-01-08T14:17:53.9276771Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-01-08T14:17:53.9276907Z 
2020-01-08T14:17:53.9276934Z 
2020-01-08T14:17:53.9282322Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-01-08T14:17:53.9282395Z Build completed unsuccessfully in 0:01:33
2020-01-08T14:17:53.9282395Z Build completed unsuccessfully in 0:01:33
2020-01-08T14:17:53.9341333Z == clock drift check ==
2020-01-08T14:17:53.9369980Z   local time: Wed Jan  8 14:17:53 UTC 2020
2020-01-08T14:17:54.3587814Z   network time: Wed, 08 Jan 2020 14:17:54 GMT
2020-01-08T14:17:54.3589029Z == end clock drift check ==
2020-01-08T14:17:55.1011900Z 
2020-01-08T14:17:55.1123616Z ##[error]Bash exited with code '1'.
2020-01-08T14:17:55.1154246Z ##[section]Starting: Checkout
2020-01-08T14:17:55.1156578Z ==============================================================================
2020-01-08T14:17:55.1156637Z Task         : Get sources
2020-01-08T14:17:55.1156687Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
