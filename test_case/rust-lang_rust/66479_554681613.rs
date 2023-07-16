plain
2019-11-16T22:33:04.4091904Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-16T22:33:04.4264895Z ##[command]git config gc.auto 0
2019-11-16T22:33:04.4319510Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-16T22:33:04.4361097Z ##[command]git config --get-all http.proxy
2019-11-16T22:33:04.4490094Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66479/merge:refs/remotes/pull/66479/merge
---
2019-11-16T22:38:48.9339961Z    Compiling serde_json v1.0.40
2019-11-16T22:38:50.5460304Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-16T22:39:00.5053113Z     Finished release [optimized] target(s) in 1m 20s
2019-11-16T22:39:00.5149738Z tidy check
2019-11-16T22:39:01.3223735Z tidy error: /checkout/src/libcore/num/mod.rs:146: undocumented unsafe
2019-11-16T22:39:01.3223852Z tidy error: /checkout/src/libcore/num/mod.rs:171: line longer than 100 chars
2019-11-16T22:39:01.3223944Z tidy error: /checkout/src/libcore/num/mod.rs:172: line longer than 100 chars
2019-11-16T22:39:01.3354519Z tidy error: /checkout/src/libcore/num/mod.rs:173: line longer than 100 chars
2019-11-16T22:39:01.3354888Z tidy error: /checkout/src/libcore/num/mod.rs:174: line longer than 100 chars
2019-11-16T22:39:01.3355079Z tidy error: /checkout/src/libcore/num/mod.rs:175: line longer than 100 chars
2019-11-16T22:39:01.3355220Z tidy error: /checkout/src/libcore/num/mod.rs:176: line longer than 100 chars
2019-11-16T22:39:01.3355382Z tidy error: /checkout/src/libcore/num/mod.rs:177: line longer than 100 chars
2019-11-16T22:39:01.3355548Z tidy error: /checkout/src/libcore/num/mod.rs:178: line longer than 100 chars
2019-11-16T22:39:01.3355689Z tidy error: /checkout/src/libcore/num/mod.rs:179: line longer than 100 chars
2019-11-16T22:39:01.3355847Z tidy error: /checkout/src/libcore/num/mod.rs:180: line longer than 100 chars
2019-11-16T22:39:01.3356070Z tidy error: /checkout/src/libcore/num/mod.rs:183: line longer than 100 chars
2019-11-16T22:39:01.3356235Z tidy error: /checkout/src/libcore/num/mod.rs:184: line longer than 100 chars
2019-11-16T22:39:01.3357646Z tidy error: /checkout/src/libcore/num/mod.rs:185: line longer than 100 chars
2019-11-16T22:39:01.3357831Z tidy error: /checkout/src/libcore/num/mod.rs:186: line longer than 100 chars
2019-11-16T22:39:01.3358036Z tidy error: /checkout/src/libcore/num/mod.rs:187: line longer than 100 chars
2019-11-16T22:39:01.3358180Z tidy error: /checkout/src/libcore/num/mod.rs:188: line longer than 100 chars
2019-11-16T22:39:01.3358333Z tidy error: /checkout/src/libcore/num/mod.rs:189: line longer than 100 chars
2019-11-16T22:39:01.3358493Z tidy error: /checkout/src/libcore/num/mod.rs:190: line longer than 100 chars
2019-11-16T22:39:01.3358860Z tidy error: /checkout/src/libcore/num/mod.rs:191: line longer than 100 chars
2019-11-16T22:39:01.3361462Z tidy error: /checkout/src/libcore/num/mod.rs:192: line longer than 100 chars
2019-11-16T22:39:02.9311215Z Found 441 error codes
2019-11-16T22:39:02.9311355Z Found 0 error codes with no tests
2019-11-16T22:39:02.9311397Z Done!
2019-11-16T22:39:02.9315550Z 
2019-11-16T22:39:02.9315550Z 
2019-11-16T22:39:02.9315648Z 
2019-11-16T22:39:02.9316489Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-16T22:39:02.9316614Z 
2019-11-16T22:39:02.9316640Z 
2019-11-16T22:39:02.9321100Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-16T22:39:02.9321181Z Build completed unsuccessfully in 0:01:23
2019-11-16T22:39:02.9321181Z Build completed unsuccessfully in 0:01:23
2019-11-16T22:39:02.9324657Z some tidy checks failed
2019-11-16T22:39:02.9365583Z == clock drift check ==
2019-11-16T22:39:02.9379012Z   local time: Sat Nov 16 22:39:02 UTC 2019
2019-11-16T22:39:03.4694735Z   network time: Sat, 16 Nov 2019 22:39:03 GMT
2019-11-16T22:39:03.4698950Z == end clock drift check ==
2019-11-16T22:39:04.8255041Z 
2019-11-16T22:39:04.8343096Z ##[error]Bash exited with code '1'.
2019-11-16T22:39:04.8367135Z ##[section]Starting: Checkout
2019-11-16T22:39:04.8368653Z ==============================================================================
2019-11-16T22:39:04.8368703Z Task         : Get sources
2019-11-16T22:39:04.8368745Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
