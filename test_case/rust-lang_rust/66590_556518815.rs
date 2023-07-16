plain
2019-11-20T22:49:12.1655171Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-20T22:49:12.1843070Z ##[command]git config gc.auto 0
2019-11-20T22:49:12.1904120Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-20T22:49:12.1960672Z ##[command]git config --get-all http.proxy
2019-11-20T22:49:12.2095924Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66590/merge:refs/remotes/pull/66590/merge
---
2019-11-20T22:58:17.5425093Z    Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2019-11-20T22:58:55.2604265Z error: unused variable: `did`
2019-11-20T22:58:55.2612023Z    --> src/librustc/traits/coherence.rs:506:20
2019-11-20T22:58:55.2612541Z     |
2019-11-20T22:58:55.2612987Z 506 |         ty::Opaque(did, _) => {
2019-11-20T22:58:55.2613465Z     |                    ^^^ help: consider prefixing with an underscore: `_did`
2019-11-20T22:58:55.2614315Z     = note: `-D unused-variables` implied by `-D warnings`
2019-11-20T22:58:55.2614483Z 
2019-11-20T22:58:55.2614836Z error: unused variable: `tcx`
2019-11-20T22:58:55.2615229Z    --> src/librustc/traits/coherence.rs:463:5
---
2019-11-20T22:59:25.2426718Z   local time: Wed Nov 20 22:59:25 UTC 2019
2019-11-20T22:59:25.2652815Z   network time: Wed, 20 Nov 2019 22:59:25 GMT
2019-11-20T22:59:25.2654555Z == end clock drift check ==
2019-11-20T22:59:26.1936364Z 
2019-11-20T22:59:26.2027474Z ##[error]Bash exited with code '1'.
2019-11-20T22:59:26.2059141Z ##[section]Starting: Checkout
2019-11-20T22:59:26.2061021Z ==============================================================================
2019-11-20T22:59:26.2061077Z Task         : Get sources
2019-11-20T22:59:26.2061140Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
