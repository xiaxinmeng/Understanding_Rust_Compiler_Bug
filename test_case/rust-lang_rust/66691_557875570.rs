plain
2019-11-24T10:09:05.7332366Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-24T10:09:05.7528559Z ##[command]git config gc.auto 0
2019-11-24T10:09:05.7581946Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-24T10:09:05.7628151Z ##[command]git config --get-all http.proxy
2019-11-24T10:09:05.7768230Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66691/merge:refs/remotes/pull/66691/merge
---
2019-11-24T10:14:49.5329718Z     Finished release [optimized] target(s) in 1m 20s
2019-11-24T10:14:49.5421883Z tidy check
2019-11-24T10:14:50.7908062Z * 588 error codes
2019-11-24T10:14:50.7909340Z * highest error code: E0744
2019-11-24T10:14:50.9099199Z tidy error: /checkout/src/libcore/macros/mod.rs:800: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9099665Z tidy error: /checkout/src/libcore/macros/mod.rs:904: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9099799Z tidy error: /checkout/src/libcore/macros/mod.rs:1289: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9100142Z tidy error: /checkout/src/libcore/macros/mod.rs:1307: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9100209Z tidy error: /checkout/src/libcore/macros/mod.rs:1321: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9100282Z tidy error: /checkout/src/libcore/macros/mod.rs:1335: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9100331Z tidy error: /checkout/src/libcore/macros/mod.rs:1356: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9100398Z tidy error: /checkout/src/libcore/macros/mod.rs:1369: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9112362Z tidy error: /checkout/src/libcore/char/convert.rs:169: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9196743Z tidy error: /checkout/src/libcore/num/bignum.rs:15: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9200452Z tidy error: /checkout/src/libcore/num/flt2dec/mod.rs:119: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9212628Z tidy error: /checkout/src/libcore/num/dec2flt/mod.rs:81: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9212784Z tidy error: /checkout/src/libcore/num/dec2flt/mod.rs:186: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9245033Z tidy error: /checkout/src/libcore/num/diy_float.rs:6: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9285874Z tidy error: /checkout/src/libcore/panicking.rs:26: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9362212Z tidy error: /checkout/src/libcore/ptr/unique.rs:30: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9362895Z tidy error: /checkout/src/libcore/ffi.rs:38: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9363922Z tidy error: /checkout/src/libcore/ffi.rs:45: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9364078Z tidy error: /checkout/src/libcore/ffi.rs:71: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9364139Z tidy error: /checkout/src/libcore/ffi.rs:93: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9364197Z tidy error: /checkout/src/libcore/ffi.rs:113: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9364274Z tidy error: /checkout/src/libcore/ffi.rs:133: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9365933Z tidy error: /checkout/src/libcore/ffi.rs:153: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9366235Z tidy error: /checkout/src/libcore/ffi.rs:171: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9366472Z tidy error: /checkout/src/libcore/ffi.rs:210: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9366740Z tidy error: /checkout/src/libcore/ffi.rs:231: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9367115Z tidy error: /checkout/src/libcore/ffi.rs:245: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9367329Z tidy error: /checkout/src/libcore/ffi.rs:260: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9367515Z tidy error: /checkout/src/libcore/ffi.rs:286: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9367708Z tidy error: /checkout/src/libcore/ffi.rs:311: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9367901Z tidy error: /checkout/src/libcore/ffi.rs:318: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9368095Z tidy error: /checkout/src/libcore/ffi.rs:326: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9368291Z tidy error: /checkout/src/libcore/ffi.rs:351: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9368657Z tidy error: /checkout/src/libcore/ffi.rs:369: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9369153Z tidy error: /checkout/src/libcore/clone.rs:150: malformed stability attribute: missing `feature` key
2019-11-24T10:14:50.9369414Z tidy error: /checkout/src/libcore/clone.rs:160: malformed stability attribute: missing `feature` key
2019-11-24T10:14:51.3648577Z tidy error: The Unstable Book has a 'library feature' section 'asm' which doesn't correspond to an unstable library feature
2019-11-24T10:14:51.3648911Z tidy error: The Unstable Book has a 'library feature' section 'c-void-variant' which doesn't correspond to an unstable library feature
2019-11-24T10:14:51.3649289Z tidy error: The Unstable Book has a 'library feature' section 'char-error-internals' which doesn't correspond to an unstable library feature
2019-11-24T10:14:51.3649929Z tidy error: The Unstable Book has a 'library feature' section 'concat-idents' which doesn't correspond to an unstable library feature
2019-11-24T10:14:51.3650213Z tidy error: The Unstable Book has a 'library feature' section 'core-panic' which doesn't correspond to an unstable library feature
2019-11-24T10:14:51.3650467Z tidy error: The Unstable Book has a 'library feature' section 'core-private-bignum' which doesn't correspond to an unstable library feature
2019-11-24T10:14:51.3650737Z tidy error: The Unstable Book has a 'library feature' section 'core-private-diy-float' which doesn't correspond to an unstable library feature
2019-11-24T10:14:51.3650988Z tidy error: The Unstable Book has a 'library feature' section 'dec2flt' which doesn't correspond to an unstable library feature
2019-11-24T10:14:51.3651234Z tidy error: The Unstable Book has a 'library feature' section 'derive-clone-copy' which doesn't correspond to an unstable library feature
2019-11-24T10:14:51.3651720Z tidy error: The Unstable Book has a 'library feature' section 'flt2dec' which doesn't correspond to an unstable library feature
2019-11-24T10:14:51.3651961Z tidy error: The Unstable Book has a 'library feature' section 'global-asm' which doesn't correspond to an unstable library feature
2019-11-24T10:14:51.3652211Z tidy error: The Unstable Book has a 'library feature' section 'trace-macros' which doesn't correspond to an unstable library feature
2019-11-24T10:14:52.0044858Z Found 485 error codes
2019-11-24T10:14:52.0044961Z Found 0 error codes with no tests
2019-11-24T10:14:52.0045007Z Done!
2019-11-24T10:14:52.0045050Z some tidy checks failed
2019-11-24T10:14:52.0045050Z some tidy checks failed
2019-11-24T10:14:52.0051665Z 
2019-11-24T10:14:52.0051747Z 
2019-11-24T10:14:52.0052559Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-24T10:14:52.0052698Z 
2019-11-24T10:14:52.0052721Z 
2019-11-24T10:14:52.0057627Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-24T10:14:52.0057876Z Build completed unsuccessfully in 0:01:24
2019-11-24T10:14:52.0057876Z Build completed unsuccessfully in 0:01:24
2019-11-24T10:14:52.0104521Z == clock drift check ==
2019-11-24T10:14:52.0115590Z   local time: Sun Nov 24 10:14:52 UTC 2019
2019-11-24T10:14:52.7133260Z   network time: Sun, 24 Nov 2019 10:14:52 GMT
2019-11-24T10:14:52.7134969Z == end clock drift check ==
2019-11-24T10:14:53.5386923Z 
2019-11-24T10:14:53.5499492Z ##[error]Bash exited with code '1'.
2019-11-24T10:14:53.5522460Z ##[section]Starting: Checkout
2019-11-24T10:14:53.5523857Z ==============================================================================
2019-11-24T10:14:53.5524088Z Task         : Get sources
2019-11-24T10:14:53.5524155Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
