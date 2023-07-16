plain
2019-10-28T02:45:49.2153974Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-28T02:45:49.2360486Z ##[command]git config gc.auto 0
2019-10-28T02:45:49.2445135Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-28T02:45:49.2501766Z ##[command]git config --get-all http.proxy
2019-10-28T02:45:49.2650670Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64736/merge:refs/remotes/pull/64736/merge
---
2019-10-28T02:52:19.1609279Z    Compiling serde_json v1.0.40
2019-10-28T02:52:21.0688096Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-28T02:52:33.0949420Z     Finished release [optimized] target(s) in 1m 33s
2019-10-28T02:52:33.1031272Z tidy check
2019-10-28T02:52:33.5928550Z tidy error: /checkout/src/librustc_mir/transform/copy_prop.rs:34: line longer than 100 chars
2019-10-28T02:52:33.5928676Z tidy error: /checkout/src/librustc_mir/transform/copy_prop.rs:103: line longer than 100 chars
2019-10-28T02:52:33.5928730Z tidy error: /checkout/src/librustc_mir/transform/copy_prop.rs:133: line longer than 100 chars
2019-10-28T02:52:33.5936556Z tidy error: /checkout/src/librustc_mir/transform/copy_prop.rs:276: line longer than 100 chars
2019-10-28T02:52:33.5936883Z tidy error: /checkout/src/librustc_mir/transform/uniform_array_move_out.rs:45: line longer than 100 chars
2019-10-28T02:52:33.5937087Z tidy error: /checkout/src/librustc_mir/transform/uniform_array_move_out.rs:223: line longer than 100 chars
2019-10-28T02:52:33.5937274Z tidy error: /checkout/src/librustc_mir/transform/dump_mir.rs:21: line longer than 100 chars
2019-10-28T02:52:33.5937435Z tidy error: /checkout/src/librustc_mir/transform/rustc_peek.rs:67: line longer than 100 chars
2019-10-28T02:52:33.5937573Z tidy error: /checkout/src/librustc_mir/transform/rustc_peek.rs:70: line longer than 100 chars
2019-10-28T02:52:33.5952661Z tidy error: /checkout/src/librustc_mir/transform/generator.rs:873: line longer than 100 chars
2019-10-28T02:52:33.5952760Z tidy error: /checkout/src/librustc_mir/transform/generator.rs:1010: line longer than 100 chars
2019-10-28T02:52:33.5952850Z tidy error: /checkout/src/librustc_mir/transform/generator.rs:1174: line longer than 100 chars
2019-10-28T02:52:33.5952911Z tidy error: /checkout/src/librustc_mir/transform/simplify.rs:60: line longer than 100 chars
2019-10-28T02:52:33.5952969Z tidy error: /checkout/src/librustc_mir/transform/simplify.rs:297: line longer than 100 chars
2019-10-28T02:52:33.5963217Z tidy error: /checkout/src/librustc_mir/transform/add_moves_for_packed_drops.rs:49: line longer than 100 chars
2019-10-28T02:52:33.5980924Z tidy error: /checkout/src/librustc_mir/transform/inline.rs:41: line longer than 100 chars
2019-10-28T02:52:33.5981031Z tidy error: /checkout/src/librustc_mir/transform/inline.rs:139: line longer than 100 chars
2019-10-28T02:52:33.5981086Z tidy error: /checkout/src/librustc_mir/transform/inline.rs:545: line longer than 100 chars
2019-10-28T02:52:33.5981140Z tidy error: /checkout/src/librustc_mir/transform/inline.rs:546: line longer than 100 chars
2019-10-28T02:52:33.5981217Z tidy error: /checkout/src/librustc_mir/transform/cleanup_post_borrowck.rs:32: line longer than 100 chars
2019-10-28T02:52:33.5981273Z tidy error: /checkout/src/librustc_mir/transform/add_call_guards.rs:34: line longer than 100 chars
2019-10-28T02:52:33.5981326Z tidy error: /checkout/src/librustc_mir/transform/const_prop.rs:41: line longer than 100 chars
2019-10-28T02:52:33.5981415Z tidy error: /checkout/src/librustc_mir/transform/deaggregator.rs:9: line longer than 100 chars
2019-10-28T02:52:33.5987634Z tidy error: /checkout/src/librustc_mir/transform/mod.rs:303: line longer than 100 chars
2019-10-28T02:52:33.6000367Z tidy error: /checkout/src/librustc_mir/util/liveness.rs:86: line longer than 100 chars
2019-10-28T02:52:33.6028569Z tidy error: /checkout/src/librustc_mir/dataflow/impls/storage_liveness.rs:90: line longer than 100 chars
2019-10-28T02:52:33.6093920Z tidy error: /checkout/src/librustc_mir/borrow_check/borrow_set.rs:109: line longer than 100 chars
2019-10-28T02:52:33.6094030Z tidy error: /checkout/src/librustc_mir/borrow_check/error_reporting.rs:375: line longer than 100 chars
2019-10-28T02:52:33.6094093Z tidy error: /checkout/src/librustc_mir/borrow_check/error_reporting.rs:528: line longer than 100 chars
2019-10-28T02:52:33.6094152Z tidy error: /checkout/src/librustc_mir/borrow_check/error_reporting.rs:541: line longer than 100 chars
2019-10-28T02:52:33.6138079Z tidy error: /checkout/src/librustc_mir/borrow_check/nll/type_check/mod.rs:167: line longer than 100 chars
2019-10-28T02:52:33.6138177Z tidy error: /checkout/src/librustc_mir/borrow_check/nll/type_check/mod.rs:168: line longer than 100 chars
2019-10-28T02:52:33.6138235Z tidy error: /checkout/src/librustc_mir/borrow_check/nll/type_check/mod.rs:534: line longer than 100 chars
2019-10-28T02:52:33.6156440Z tidy error: /checkout/src/librustc_mir/borrow_check/nll/mod.rs:299: line longer than 100 chars
2019-10-28T02:52:33.6165178Z tidy error: /checkout/src/librustc_mir/borrow_check/mutability_errors.rs:67: line longer than 100 chars
2019-10-28T02:52:33.6180173Z tidy error: /checkout/src/librustc_mir/borrow_check/conflict_errors.rs:208: line longer than 100 chars
2019-10-28T02:52:33.6180264Z tidy error: /checkout/src/librustc_mir/borrow_check/conflict_errors.rs:311: line longer than 100 chars
2019-10-28T02:52:33.6189470Z tidy error: /checkout/src/librustc_mir/borrow_check/conflict_errors.rs:587: line longer than 100 chars
2019-10-28T02:52:33.6189615Z tidy error: /checkout/src/librustc_mir/borrow_check/conflict_errors.rs:1008: line longer than 100 chars
2019-10-28T02:52:33.6189670Z tidy error: /checkout/src/librustc_mir/borrow_check/conflict_errors.rs:1088: line longer than 100 chars
2019-10-28T02:52:33.6189723Z tidy error: /checkout/src/librustc_mir/borrow_check/conflict_errors.rs:1119: line longer than 100 chars
2019-10-28T02:52:33.6189794Z tidy error: /checkout/src/librustc_mir/borrow_check/conflict_errors.rs:1562: line longer than 100 chars
2019-10-28T02:52:33.6189846Z tidy error: /checkout/src/librustc_mir/borrow_check/conflict_errors.rs:1570: line longer than 100 chars
2019-10-28T02:52:33.6189898Z tidy error: /checkout/src/librustc_mir/borrow_check/conflict_errors.rs:1644: line longer than 100 chars
2019-10-28T02:52:33.6190012Z tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:157: line longer than 100 chars
2019-10-28T02:52:33.6190064Z tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:290: line longer than 100 chars
2019-10-28T02:52:33.6190145Z tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:327: line longer than 100 chars
2019-10-28T02:52:33.6190199Z tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1293: line longer than 100 chars
2019-10-28T02:52:33.6236446Z tidy error: /checkout/src/librustc_mir/shim.rs:170: line longer than 100 chars
2019-10-28T02:52:33.8262859Z tidy error: /checkout/src/librustc/mir/cache.rs:46: line longer than 100 chars
2019-10-28T02:52:33.8262985Z tidy error: /checkout/src/librustc/mir/cache.rs:122: line longer than 100 chars
2019-10-28T02:52:33.8301301Z tidy error: /checkout/src/librustc/mir/mod.rs:41: TODO is deprecated; use FIXME
2019-10-28T02:52:33.9203366Z tidy error: /checkout/src/librustc_codegen_ssa/mir/place.rs:589: line longer than 100 chars
2019-10-28T02:52:35.5973108Z some tidy checks failed
2019-10-28T02:52:35.5973217Z Found 484 error codes
2019-10-28T02:52:35.5973294Z Found 0 error codes with no tests
2019-10-28T02:52:35.5973393Z Done!
2019-10-28T02:52:35.5973393Z Done!
2019-10-28T02:52:35.5977679Z 
2019-10-28T02:52:35.5977972Z 
2019-10-28T02:52:35.5979096Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-28T02:52:35.5979221Z 
2019-10-28T02:52:35.5979246Z 
2019-10-28T02:52:35.5994092Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-28T02:52:35.5994177Z Build completed unsuccessfully in 0:01:37
2019-10-28T02:52:35.5994177Z Build completed unsuccessfully in 0:01:37
2019-10-28T02:52:35.6047330Z == clock drift check ==
2019-10-28T02:52:35.6056384Z   local time: Mon Oct 28 02:52:35 UTC 2019
2019-10-28T02:52:35.7419032Z   network time: Mon, 28 Oct 2019 02:52:35 GMT
2019-10-28T02:52:35.7419150Z == end clock drift check ==
2019-10-28T02:52:37.0864148Z 
2019-10-28T02:52:37.0967858Z ##[error]Bash exited with code '1'.
2019-10-28T02:52:37.1002013Z ##[section]Starting: Checkout
2019-10-28T02:52:37.1004241Z ==============================================================================
2019-10-28T02:52:37.1004318Z Task         : Get sources
2019-10-28T02:52:37.1004367Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
