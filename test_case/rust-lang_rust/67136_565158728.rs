plain
2019-12-12T19:47:45.0485160Z [RUSTC-TIMING] build_script_build test:false 0.920
2019-12-12T19:47:52.7953620Z error: `sync::atomic::AtomicI128::new` is not yet stable as a const fn
2019-12-12T19:47:52.7954470Z     --> src/libcore/sync/atomic.rs:1221:48
2019-12-12T19:47:52.7955030Z      |
2019-12-12T19:47:52.7955660Z 1180 | / macro_rules! atomic_int {
2019-12-12T19:47:52.7956260Z 1181 | |     ($cfg_cas:meta,
2019-12-12T19:47:52.7956880Z 1182 | |      $stable:meta,
2019-12-12T19:47:52.7957540Z 1183 | |      $stable_cxchg:meta,
2019-12-12T19:47:52.7958090Z ...    |
2019-12-12T19:47:52.7958730Z 1221 | |         pub const $atomic_init: $atomic_type = $atomic_type::new(0);
2019-12-12T19:47:52.7959980Z ...    |
2019-12-12T19:47:52.7960570Z 1973 | |     }
2019-12-12T19:47:52.7961130Z 1974 | | }
2019-12-12T19:47:52.7961760Z      | |_- in this expansion of `atomic_int!`
2019-12-12T19:47:52.7961760Z      | |_- in this expansion of `atomic_int!`
2019-12-12T19:47:52.7962420Z ...
2019-12-12T19:47:52.7963000Z 2121 | / atomic_int! {
2019-12-12T19:47:52.7964480Z 2122 | |     cfg(target_has_atomic = "128"),
2019-12-12T19:47:52.7965220Z 2123 | |     unstable(feature = "integer_atomics", issue = "32976"),
2019-12-12T19:47:52.7965940Z 2124 | |     unstable(feature = "integer_atomics", issue = "32976"),
2019-12-12T19:47:52.7966510Z ...    |
2019-12-12T19:47:52.7967140Z 2136 | |     i128 AtomicI128 ATOMIC_I128_INIT
2019-12-12T19:47:52.7968370Z      | |_- in this macro invocation
2019-12-12T19:47:52.7968900Z      |
2019-12-12T19:47:52.7969530Z      = help: add `#![feature(const_integer_atomics)]` to the crate attributes to enable
2019-12-12T19:47:52.7969830Z 
2019-12-12T19:47:52.7969830Z 
2019-12-12T19:47:52.8354650Z error: `sync::atomic::AtomicU128::new` is not yet stable as a const fn
2019-12-12T19:47:52.8355400Z     --> src/libcore/sync/atomic.rs:1221:48
2019-12-12T19:47:52.8355950Z      |
2019-12-12T19:47:52.8356570Z 1180 | / macro_rules! atomic_int {
2019-12-12T19:47:52.8357180Z 1181 | |     ($cfg_cas:meta,
2019-12-12T19:47:52.8357810Z 1182 | |      $stable:meta,
2019-12-12T19:47:52.8358460Z 1183 | |      $stable_cxchg:meta,
2019-12-12T19:47:52.8359040Z ...    |
2019-12-12T19:47:52.8359720Z 1221 | |         pub const $atomic_init: $atomic_type = $atomic_type::new(0);
2019-12-12T19:47:52.8360980Z ...    |
2019-12-12T19:47:52.8361560Z 1973 | |     }
2019-12-12T19:47:52.8362170Z 1974 | | }
2019-12-12T19:47:52.8362820Z      | |_- in this expansion of `atomic_int!`
2019-12-12T19:47:52.8362820Z      | |_- in this expansion of `atomic_int!`
2019-12-12T19:47:52.8363340Z ...
2019-12-12T19:47:52.8363950Z 2139 | / atomic_int! {
2019-12-12T19:47:52.8364590Z 2140 | |     cfg(target_has_atomic = "128"),
2019-12-12T19:47:52.8365280Z 2141 | |     unstable(feature = "integer_atomics", issue = "32976"),
2019-12-12T19:47:52.8365960Z 2142 | |     unstable(feature = "integer_atomics", issue = "32976"),
2019-12-12T19:47:52.8366520Z ...    |
2019-12-12T19:47:52.8367160Z 2154 | |     u128 AtomicU128 ATOMIC_U128_INIT
2019-12-12T19:47:52.8368700Z      | |_- in this macro invocation
2019-12-12T19:47:52.8369250Z      |
2019-12-12T19:47:52.8369860Z      = help: add `#![feature(const_integer_atomics)]` to the crate attributes to enable
2019-12-12T19:47:52.8369960Z 
---
2019-12-12T19:49:01.4101010Z   local time: Thu Dec 12 19:49:01 UTC 2019
2019-12-12T19:49:01.4654960Z   network time: Thu, 12 Dec 2019 19:49:01 GMT
2019-12-12T19:49:01.4656250Z == end clock drift check ==
2019-12-12T19:49:01.4702650Z 
2019-12-12T19:49:01.4830100Z ##[error]Bash exited with code '1'.
2019-12-12T19:49:01.4873710Z ##[section]Starting: Checkout
2019-12-12T19:49:01.4876260Z ==============================================================================
2019-12-12T19:49:01.4876370Z Task         : Get sources
2019-12-12T19:49:01.4876450Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
