plain
2019-08-29T23:24:43.6838383Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-29T23:24:43.7040191Z ##[command]git config gc.auto 0
2019-08-29T23:24:43.7124890Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-29T23:24:43.7174202Z ##[command]git config --get-all http.proxy
2019-08-29T23:24:43.7315436Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64007/merge:refs/remotes/pull/64007/merge
---
2019-08-30T00:00:53.2149461Z    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
2019-08-30T00:00:56.8983572Z error: multiple patterns covering the same range
2019-08-30T00:00:56.8984603Z    --> src/librustc_target/abi/mod.rs:494:13
2019-08-30T00:00:56.8985035Z     |
2019-08-30T00:00:56.8985977Z 493 |             -0x0000_0000_0000_0080..=0x0000_0000_0000_007f => I8,
2019-08-30T00:00:56.8986705Z     |             ---------------------------------------------- this range overlaps on `-128i128..=127i128`
2019-08-30T00:00:56.8987629Z 494 |             -0x0000_0000_0000_8000..=0x0000_0000_0000_7fff => I16,
2019-08-30T00:00:56.8988345Z     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overlapping patterns
2019-08-30T00:00:56.8989197Z     = note: `-D unreachable-patterns` implied by `-D warnings`
2019-08-30T00:00:56.8993751Z 
2019-08-30T00:00:56.9038160Z error: multiple patterns covering the same range
2019-08-30T00:00:56.9038720Z    --> src/librustc_target/abi/mod.rs:495:13
2019-08-30T00:00:56.9038720Z    --> src/librustc_target/abi/mod.rs:495:13
2019-08-30T00:00:56.9039143Z     |
2019-08-30T00:00:56.9039771Z 493 |             -0x0000_0000_0000_0080..=0x0000_0000_0000_007f => I8,
2019-08-30T00:00:56.9040370Z     |             ---------------------------------------------- this range overlaps on `-128i128..=127i128`
2019-08-30T00:00:56.9041465Z 494 |             -0x0000_0000_0000_8000..=0x0000_0000_0000_7fff => I16,
2019-08-30T00:00:56.9042043Z     |             ---------------------------------------------- this range overlaps on `-32768i128..=32767i128`
2019-08-30T00:00:56.9042972Z 495 |             -0x0000_0000_8000_0000..=0x0000_0000_7fff_ffff => I32,
2019-08-30T00:00:56.9043460Z     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overlapping patterns
2019-08-30T00:00:56.9118088Z error: multiple patterns covering the same range
2019-08-30T00:00:56.9118386Z    --> src/librustc_target/abi/mod.rs:496:13
2019-08-30T00:00:56.9118603Z     |
2019-08-30T00:00:56.9118603Z     |
2019-08-30T00:00:56.9118937Z 493 |             -0x0000_0000_0000_0080..=0x0000_0000_0000_007f => I8,
2019-08-30T00:00:56.9119300Z     |             ---------------------------------------------- this range overlaps on `-128i128..=127i128`
2019-08-30T00:00:56.9119641Z 494 |             -0x0000_0000_0000_8000..=0x0000_0000_0000_7fff => I16,
2019-08-30T00:00:56.9120177Z     |             ---------------------------------------------- this range overlaps on `-32768i128..=32767i128`
2019-08-30T00:00:56.9120647Z 495 |             -0x0000_0000_8000_0000..=0x0000_0000_7fff_ffff => I32,
2019-08-30T00:00:56.9121083Z     |             ---------------------------------------------- this range overlaps on `-2147483648i128..=2147483647i128`
2019-08-30T00:00:56.9121385Z 496 |             -0x8000_0000_0000_0000..=0x7fff_ffff_ffff_ffff => I64,
2019-08-30T00:00:56.9121834Z     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ overlapping patterns
2019-08-30T00:00:56.9167451Z error: multiple patterns covering the same range
2019-08-30T00:00:56.9167785Z    --> src/librustc_target/abi/mod.rs:505:13
2019-08-30T00:00:56.9168028Z     |
2019-08-30T00:00:56.9168028Z     |
2019-08-30T00:00:56.9168323Z 504 |             0..=0x0000_0000_0000_00ff => I8,
2019-08-30T00:00:56.9169093Z     |             ------------------------- this range overlaps on `0u128..=255u128`
2019-08-30T00:00:56.9169393Z 505 |             0..=0x0000_0000_0000_ffff => I16,
2019-08-30T00:00:56.9169904Z     |             ^^^^^^^^^^^^^^^^^^^^^^^^^ overlapping patterns
2019-08-30T00:00:56.9227068Z error: multiple patterns covering the same range
2019-08-30T00:00:56.9227399Z    --> src/librustc_target/abi/mod.rs:506:13
2019-08-30T00:00:56.9227670Z     |
2019-08-30T00:00:56.9227670Z     |
2019-08-30T00:00:56.9227990Z 504 |             0..=0x0000_0000_0000_00ff => I8,
2019-08-30T00:00:56.9228377Z     |             ------------------------- this range overlaps on `0u128..=255u128`
2019-08-30T00:00:56.9228672Z 505 |             0..=0x0000_0000_0000_ffff => I16,
2019-08-30T00:00:56.9229243Z     |             ------------------------- this range overlaps on `0u128..=65535u128`
2019-08-30T00:00:56.9229827Z 506 |             0..=0x0000_0000_ffff_ffff => I32,
2019-08-30T00:00:56.9230170Z     |             ^^^^^^^^^^^^^^^^^^^^^^^^^ overlapping patterns
2019-08-30T00:00:56.9261946Z error: multiple patterns covering the same range
2019-08-30T00:00:56.9262273Z    --> src/librustc_target/abi/mod.rs:507:13
2019-08-30T00:00:56.9262814Z     |
2019-08-30T00:00:56.9262814Z     |
2019-08-30T00:00:56.9263142Z 504 |             0..=0x0000_0000_0000_00ff => I8,
2019-08-30T00:00:56.9263509Z     |             ------------------------- this range overlaps on `0u128..=255u128`
2019-08-30T00:00:56.9263856Z 505 |             0..=0x0000_0000_0000_ffff => I16,
2019-08-30T00:00:56.9264226Z     |             ------------------------- this range overlaps on `0u128..=65535u128`
2019-08-30T00:00:56.9264709Z 506 |             0..=0x0000_0000_ffff_ffff => I32,
2019-08-30T00:00:56.9265162Z     |             ------------------------- this range overlaps on `0u128..=4294967295u128`
2019-08-30T00:00:56.9265467Z 507 |             0..=0xffff_ffff_ffff_ffff => I64,
2019-08-30T00:00:56.9265827Z     |             ^^^^^^^^^^^^^^^^^^^^^^^^^ overlapping patterns
2019-08-30T00:00:58.4044166Z error: aborting due to 6 previous errors
2019-08-30T00:00:58.4049163Z 
2019-08-30T00:00:58.4526332Z error: Could not compile `rustc_target`.
2019-08-30T00:00:58.4545539Z warning: build failed, waiting for other jobs to finish...
---
2019-08-30T00:01:00.2008025Z == clock drift check ==
2019-08-30T00:01:00.2025703Z   local time: Fri Aug 30 00:01:00 UTC 2019
2019-08-30T00:01:00.2905893Z   network time: Fri, 30 Aug 2019 00:01:00 GMT
2019-08-30T00:01:00.2910698Z == end clock drift check ==
2019-08-30T00:01:01.6587570Z ##[error]Bash exited with code '1'.
2019-08-30T00:01:01.6630166Z ##[section]Starting: Checkout
2019-08-30T00:01:01.6631826Z ==============================================================================
2019-08-30T00:01:01.6631877Z Task         : Get sources
2019-08-30T00:01:01.6631927Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
