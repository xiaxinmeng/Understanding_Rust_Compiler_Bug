plain
2019-10-25T14:01:59.3105330Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-25T14:01:59.3310034Z ##[command]git config gc.auto 0
2019-10-25T14:01:59.3375726Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-25T14:01:59.3415781Z ##[command]git config --get-all http.proxy
2019-10-25T14:01:59.3581039Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64736/merge:refs/remotes/pull/64736/merge
---
2019-10-25T14:08:45.2571948Z    Compiling serde_json v1.0.40
2019-10-25T14:08:47.1278807Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-25T14:08:58.9189928Z     Finished release [optimized] target(s) in 1m 31s
2019-10-25T14:08:58.9271624Z tidy check
2019-10-25T14:08:59.4041583Z tidy error: /checkout/src/librustc_mir/util/def_use.rs:50: line longer than 100 chars
2019-10-25T14:08:59.4041743Z tidy error: /checkout/src/librustc_mir/util/liveness.rs:86: line longer than 100 chars
2019-10-25T14:08:59.4127541Z tidy error: /checkout/src/librustc_mir/borrow_check/borrow_set.rs:109: line longer than 100 chars
2019-10-25T14:08:59.4127683Z tidy error: /checkout/src/librustc_mir/borrow_check/error_reporting.rs:375: line longer than 100 chars
2019-10-25T14:08:59.4127751Z tidy error: /checkout/src/librustc_mir/borrow_check/error_reporting.rs:528: line longer than 100 chars
2019-10-25T14:08:59.4127866Z tidy error: /checkout/src/librustc_mir/borrow_check/error_reporting.rs:541: line longer than 100 chars
2019-10-25T14:08:59.4178256Z tidy error: /checkout/src/librustc_mir/borrow_check/nll/type_check/mod.rs:167: line longer than 100 chars
2019-10-25T14:08:59.4178366Z tidy error: /checkout/src/librustc_mir/borrow_check/nll/type_check/mod.rs:168: line longer than 100 chars
2019-10-25T14:08:59.4178428Z tidy error: /checkout/src/librustc_mir/borrow_check/nll/type_check/mod.rs:534: line longer than 100 chars
2019-10-25T14:08:59.4196339Z tidy error: /checkout/src/librustc_mir/borrow_check/nll/mod.rs:299: line longer than 100 chars
2019-10-25T14:08:59.4206252Z tidy error: /checkout/src/librustc_mir/borrow_check/mutability_errors.rs:70: line longer than 100 chars
2019-10-25T14:08:59.4221968Z tidy error: /checkout/src/librustc_mir/borrow_check/conflict_errors.rs:208: line longer than 100 chars
2019-10-25T14:08:59.4222075Z tidy error: /checkout/src/librustc_mir/borrow_check/conflict_errors.rs:314: line longer than 100 chars
2019-10-25T14:08:59.4222132Z tidy error: /checkout/src/librustc_mir/borrow_check/conflict_errors.rs:590: line longer than 100 chars
2019-10-25T14:08:59.4222187Z tidy error: /checkout/src/librustc_mir/borrow_check/conflict_errors.rs:1009: line longer than 100 chars
2019-10-25T14:08:59.4222258Z tidy error: /checkout/src/librustc_mir/borrow_check/conflict_errors.rs:1089: line longer than 100 chars
2019-10-25T14:08:59.4222327Z tidy error: /checkout/src/librustc_mir/borrow_check/conflict_errors.rs:1120: line longer than 100 chars
2019-10-25T14:08:59.4222381Z tidy error: /checkout/src/librustc_mir/borrow_check/conflict_errors.rs:1570: line longer than 100 chars
2019-10-25T14:08:59.4222460Z tidy error: /checkout/src/librustc_mir/borrow_check/conflict_errors.rs:1578: line longer than 100 chars
2019-10-25T14:08:59.4228063Z tidy error: /checkout/src/librustc_mir/borrow_check/conflict_errors.rs:1655: line longer than 100 chars
2019-10-25T14:08:59.4228209Z tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:153: line longer than 100 chars
2019-10-25T14:08:59.4228296Z tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:158: line longer than 100 chars
2019-10-25T14:08:59.4228353Z tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:291: line longer than 100 chars
2019-10-25T14:08:59.4228425Z tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:328: line longer than 100 chars
2019-10-25T14:08:59.4279995Z tidy error: /checkout/src/librustc_mir/shim.rs:252: TODO is deprecated; use FIXME
2019-10-25T14:08:59.6642750Z tidy error: /checkout/src/librustc/mir/cache.rs:46: line longer than 100 chars
2019-10-25T14:08:59.6642916Z tidy error: /checkout/src/librustc/mir/cache.rs:122: line longer than 100 chars
2019-10-25T14:08:59.6643223Z tidy error: /checkout/src/librustc/mir/cache.rs: missing trailing newline
2019-10-25T14:08:59.7539186Z tidy error: /checkout/src/librustc_codegen_ssa/base.rs:375: TODO is deprecated; use FIXME
2019-10-25T14:08:59.7559028Z tidy error: /checkout/src/librustc_codegen_ssa/mir/place.rs:589: line longer than 100 chars
2019-10-25T14:09:01.4175035Z some tidy checks failed
2019-10-25T14:09:01.4175161Z Found 484 error codes
2019-10-25T14:09:01.4175205Z Found 0 error codes with no tests
2019-10-25T14:09:01.4175241Z Done!
2019-10-25T14:09:01.4175241Z Done!
2019-10-25T14:09:01.4180027Z 
2019-10-25T14:09:01.4180460Z 
2019-10-25T14:09:01.4181588Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-25T14:09:01.4181978Z 
2019-10-25T14:09:01.4182077Z 
2019-10-25T14:09:01.4182211Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-25T14:09:01.4182356Z Build completed unsuccessfully in 0:01:35
2019-10-25T14:09:01.4182356Z Build completed unsuccessfully in 0:01:35
2019-10-25T14:09:01.4235780Z == clock drift check ==
2019-10-25T14:09:01.4261512Z   local time: Fri Oct 25 14:09:01 UTC 2019
2019-10-25T14:09:01.6385120Z   network time: Fri, 25 Oct 2019 14:09:01 GMT
2019-10-25T14:09:01.6385731Z == end clock drift check ==
2019-10-25T14:09:03.2478110Z 
2019-10-25T14:09:03.2598210Z ##[error]Bash exited with code '1'.
2019-10-25T14:09:03.2671968Z ##[section]Starting: Checkout
2019-10-25T14:09:03.2673572Z ==============================================================================
2019-10-25T14:09:03.2673653Z Task         : Get sources
2019-10-25T14:09:03.2673690Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
