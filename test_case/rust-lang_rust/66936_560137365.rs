plain
2019-12-01T17:43:24.6648700Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-01T17:43:24.6831192Z ##[command]git config gc.auto 0
2019-12-01T17:43:24.6900658Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-01T17:43:24.6949477Z ##[command]git config --get-all http.proxy
2019-12-01T17:43:24.7074094Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66936/merge:refs/remotes/pull/66936/merge
---
2019-12-01T17:48:28.1541685Z    Compiling serde_json v1.0.40
2019-12-01T17:48:29.4938081Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-01T17:48:38.0405541Z     Finished release [optimized] target(s) in 1m 08s
2019-12-01T17:48:38.0486442Z tidy check
2019-12-01T17:48:38.5129187Z tidy error: /checkout/src/librustc_passes/dead.rs:130: line longer than 100 chars
2019-12-01T17:48:38.5624269Z tidy error: /checkout/src/librustc_mir/build/mod.rs:547: line longer than 100 chars
2019-12-01T17:48:38.5886670Z tidy error: /checkout/src/librustc_mir/hair/pattern/check_match.rs:129: line longer than 100 chars
2019-12-01T17:48:38.5886765Z tidy error: /checkout/src/librustc_mir/hair/pattern/check_match.rs:330: line longer than 100 chars
2019-12-01T17:48:38.6422534Z tidy error: /checkout/src/librustc/traits/error_reporting.rs:1265: line longer than 100 chars
2019-12-01T17:48:38.6638564Z tidy error: /checkout/src/librustc/hir/lowering/expr.rs:23: line longer than 100 chars
2019-12-01T17:48:38.6638805Z tidy error: /checkout/src/librustc/hir/lowering/expr.rs:118: line longer than 100 chars
2019-12-01T17:48:38.6638894Z tidy error: /checkout/src/librustc/hir/lowering/expr.rs:125: line longer than 100 chars
2019-12-01T17:48:38.6638937Z tidy error: /checkout/src/librustc/hir/lowering/expr.rs:127: line longer than 100 chars
2019-12-01T17:48:38.6638998Z tidy error: /checkout/src/librustc/hir/lowering/expr.rs:146: line longer than 100 chars
2019-12-01T17:48:38.6639043Z tidy error: /checkout/src/librustc/hir/lowering/expr.rs:156: line longer than 100 chars
2019-12-01T17:48:38.6639085Z tidy error: /checkout/src/librustc/hir/lowering/expr.rs:161: line longer than 100 chars
2019-12-01T17:48:38.6639148Z tidy error: /checkout/src/librustc/hir/lowering/expr.rs:466: line longer than 100 chars
2019-12-01T17:48:38.6639190Z tidy error: /checkout/src/librustc/hir/lowering/expr.rs:501: line longer than 100 chars
2019-12-01T17:48:38.6639241Z tidy error: /checkout/src/librustc/hir/lowering/expr.rs:603: line longer than 100 chars
2019-12-01T17:48:38.6639307Z tidy error: /checkout/src/librustc/hir/lowering/expr.rs:1246: line longer than 100 chars
2019-12-01T17:48:38.6639351Z tidy error: /checkout/src/librustc/hir/lowering/expr.rs:1284: line longer than 100 chars
2019-12-01T17:48:38.6639396Z tidy error: /checkout/src/librustc/hir/lowering/expr.rs:1304: line longer than 100 chars
2019-12-01T17:48:38.6639458Z tidy error: /checkout/src/librustc/hir/lowering/expr.rs:1390: line longer than 100 chars
2019-12-01T17:48:38.6639500Z tidy error: /checkout/src/librustc/hir/lowering/expr.rs:1415: line longer than 100 chars
2019-12-01T17:48:38.6639544Z tidy error: /checkout/src/librustc/hir/lowering/expr.rs:1434: line longer than 100 chars
2019-12-01T17:48:38.6639603Z tidy error: /checkout/src/librustc/hir/lowering/expr.rs:1479: line longer than 100 chars
2019-12-01T17:48:38.6647305Z tidy error: /checkout/src/librustc/hir/lowering/item.rs:1041: line longer than 100 chars
2019-12-01T17:48:38.6702539Z tidy error: /checkout/src/librustc/hir/lowering.rs:2135: line longer than 100 chars
2019-12-01T17:48:38.6707097Z tidy error: /checkout/src/librustc/hir/lowering.rs:2929: line longer than 100 chars
2019-12-01T17:48:38.8046628Z tidy error: /checkout/src/librustc_typeck/check/expr.rs:130: line longer than 100 chars
2019-12-01T17:48:38.8057112Z tidy error: /checkout/src/librustc_typeck/check/_match.rs:193: line longer than 100 chars
2019-12-01T17:48:38.8131649Z tidy error: /checkout/src/librustc_typeck/check/regionck.rs:1016: line longer than 100 chars
2019-12-01T17:48:38.8147765Z tidy error: /checkout/src/librustc_typeck/check/demand.rs:139: line longer than 100 chars
2019-12-01T17:48:38.8156112Z tidy error: /checkout/src/librustc_typeck/check/pat.rs:33: line longer than 100 chars
2019-12-01T17:48:38.8156486Z tidy error: /checkout/src/librustc_typeck/check/pat.rs:117: line longer than 100 chars
2019-12-01T17:48:38.8228189Z tidy error: /checkout/src/librustc_typeck/check/mod.rs:4280: line longer than 100 chars
2019-12-01T17:48:38.8234674Z tidy error: /checkout/src/librustc_typeck/check/mod.rs:5011: line longer than 100 chars
2019-12-01T17:48:38.8299773Z tidy error: /checkout/src/librustc_typeck/mem_categorization.rs:565: line longer than 100 chars
2019-12-01T17:48:38.8300104Z tidy error: /checkout/src/librustc_typeck/mem_categorization.rs:572: line longer than 100 chars
2019-12-01T17:48:40.0767416Z Found 486 error codes
2019-12-01T17:48:40.0767585Z Found 0 error codes with no tests
2019-12-01T17:48:40.0767622Z Done!
2019-12-01T17:48:40.0767678Z some tidy checks failed
2019-12-01T17:48:40.0767678Z some tidy checks failed
2019-12-01T17:48:40.0770695Z 
2019-12-01T17:48:40.0770958Z 
2019-12-01T17:48:40.0771889Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-01T17:48:40.0775277Z 
2019-12-01T17:48:40.0775299Z 
2019-12-01T17:48:40.0775462Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-01T17:48:40.0775513Z Build completed unsuccessfully in 0:01:11
2019-12-01T17:48:40.0775513Z Build completed unsuccessfully in 0:01:11
2019-12-01T17:48:40.0816397Z == clock drift check ==
2019-12-01T17:48:40.0842952Z   local time: Sun Dec  1 17:48:40 UTC 2019
2019-12-01T17:48:40.3592297Z   network time: Sun, 01 Dec 2019 17:48:40 GMT
2019-12-01T17:48:40.3593244Z == end clock drift check ==
2019-12-01T17:48:41.7573712Z 
2019-12-01T17:48:41.7627825Z ##[error]Bash exited with code '1'.
2019-12-01T17:48:41.7649755Z ##[section]Starting: Checkout
2019-12-01T17:48:41.7651201Z ==============================================================================
2019-12-01T17:48:41.7651243Z Task         : Get sources
2019-12-01T17:48:41.7651279Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
