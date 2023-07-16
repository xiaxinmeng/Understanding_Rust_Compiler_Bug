plain
2019-11-30T00:26:37.2012188Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-30T00:26:37.8786863Z ##[command]git config gc.auto 0
2019-11-30T00:26:37.8792464Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-30T00:26:37.8798195Z ##[command]git config --get-all http.proxy
2019-11-30T00:26:37.8805080Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66884/merge:refs/remotes/pull/66884/merge
---
2019-11-30T00:33:15.3507135Z    Compiling compiler_builtins v0.1.18
2019-11-30T00:33:15.6148115Z error[E0723]: loops and conditional expressions are not stable in const fn
2019-11-30T00:33:15.6148771Z     --> src/libcore/num/mod.rs:851:16
2019-11-30T00:33:15.6149058Z      |
2019-11-30T00:33:15.6152967Z 238  | / macro_rules! int_impl {
2019-11-30T00:33:15.6153390Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-11-30T00:33:15.6153829Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-11-30T00:33:15.6154188Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-11-30T00:33:15.6154464Z ...    |
2019-11-30T00:33:15.6154829Z 851  | |             if rhs == 0 || (self == Self::min_value() && rhs == -1) {
2019-11-30T00:33:15.6155461Z ...    |
2019-11-30T00:33:15.6155763Z 2394 | |     }
2019-11-30T00:33:15.6156049Z 2395 | | }
2019-11-30T00:33:15.6156049Z 2395 | | }
2019-11-30T00:33:15.6156386Z      | |_- in this expansion of `int_impl!`
2019-11-30T00:33:15.6156601Z ...
2019-11-30T00:33:15.6157002Z 2399 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-11-30T00:33:15.6157537Z 2400 | |         "[0x12]", "[0x12]", "", "" }
2019-11-30T00:33:15.6158220Z      |
2019-11-30T00:33:15.6158220Z      |
2019-11-30T00:33:15.6158926Z      = note: for more information, see issue ***/issues/57563
2019-11-30T00:33:15.6159290Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-11-30T00:33:15.6973143Z error[E0723]: loops and conditional expressions are not stable in const fn
2019-11-30T00:33:15.6973532Z     --> src/libcore/num/mod.rs:851:16
2019-11-30T00:33:15.6973835Z      |
2019-11-30T00:33:15.6974161Z 238  | / macro_rules! int_impl {
2019-11-30T00:33:15.6974161Z 238  | / macro_rules! int_impl {
2019-11-30T00:33:15.6974556Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-11-30T00:33:15.6975290Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-11-30T00:33:15.6975650Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-11-30T00:33:15.6975939Z ...    |
2019-11-30T00:33:15.6976393Z 851  | |             if rhs == 0 || (self == Self::min_value() && rhs == -1) {
2019-11-30T00:33:15.6977047Z ...    |
2019-11-30T00:33:15.6979102Z 2394 | |     }
2019-11-30T00:33:15.6979494Z 2395 | | }
2019-11-30T00:33:15.6979494Z 2395 | | }
2019-11-30T00:33:15.6979821Z      | |_- in this expansion of `int_impl!`
2019-11-30T00:33:15.6980037Z ...
2019-11-30T00:33:15.6980470Z 2405 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
2019-11-30T00:33:15.6980846Z 2406 | |         "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
2019-11-30T00:33:15.6981494Z      |
2019-11-30T00:33:15.6981494Z      |
2019-11-30T00:33:15.6982012Z      = note: for more information, see issue ***/issues/57563
2019-11-30T00:33:15.6982350Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-11-30T00:33:16.5059221Z error[E0723]: loops and conditional expressions are not stable in const fn
2019-11-30T00:33:16.5060002Z     --> src/libcore/num/mod.rs:851:16
2019-11-30T00:33:16.5060303Z      |
2019-11-30T00:33:16.5060616Z 238  | / macro_rules! int_impl {
2019-11-30T00:33:16.5060616Z 238  | / macro_rules! int_impl {
2019-11-30T00:33:16.5061107Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-11-30T00:33:16.5061512Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-11-30T00:33:16.5061926Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-11-30T00:33:16.5062186Z ...    |
2019-11-30T00:33:16.5062606Z 851  | |             if rhs == 0 || (self == Self::min_value() && rhs == -1) {
2019-11-30T00:33:16.5063378Z ...    |
2019-11-30T00:33:16.5063734Z 2394 | |     }
2019-11-30T00:33:16.5064027Z 2395 | | }
2019-11-30T00:33:16.5064027Z 2395 | | }
2019-11-30T00:33:16.5064342Z      | |_- in this expansion of `int_impl!`
2019-11-30T00:33:16.5064615Z ...
2019-11-30T00:33:16.5065022Z 2411 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
2019-11-30T00:33:16.5065465Z 2412 | |         "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
2019-11-30T00:33:16.5065834Z 2413 | |         "[0x12, 0x34, 0x56, 0x78]", "", "" }
2019-11-30T00:33:16.5066455Z      |
2019-11-30T00:33:16.5066455Z      |
2019-11-30T00:33:16.5113219Z      = note: for more information, see issue ***/issues/57563
2019-11-30T00:33:16.5113850Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-11-30T00:33:16.5114244Z error[E0723]: loops and conditional expressions are not stable in const fn
2019-11-30T00:33:16.5114524Z     --> src/libcore/num/mod.rs:851:16
2019-11-30T00:33:16.5114762Z      |
2019-11-30T00:33:16.5115092Z 238  | / macro_rules! int_impl {
2019-11-30T00:33:16.5115092Z 238  | / macro_rules! int_impl {
2019-11-30T00:33:16.5115480Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-11-30T00:33:16.5115872Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-11-30T00:33:16.5116403Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-11-30T00:33:16.5116718Z ...    |
2019-11-30T00:33:16.5117098Z 851  | |             if rhs == 0 || (self == Self::min_value() && rhs == -1) {
2019-11-30T00:33:16.5117724Z ...    |
2019-11-30T00:33:16.5118024Z 2394 | |     }
2019-11-30T00:33:16.5118602Z 2395 | | }
2019-11-30T00:33:16.5118602Z 2395 | | }
2019-11-30T00:33:16.5118935Z      | |_- in this expansion of `int_impl!`
2019-11-30T00:33:16.5119168Z ...
2019-11-30T00:33:16.5119556Z 2418 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
2019-11-30T00:33:16.5120102Z 2419 | |          "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
2019-11-30T00:33:16.5120517Z 2420 | |          "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2019-11-30T00:33:16.5120902Z 2421 | |          "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
2019-11-30T00:33:16.5121540Z      |
2019-11-30T00:33:16.5121540Z      |
2019-11-30T00:33:16.5121940Z      = note: for more information, see issue ***/issues/57563
2019-11-30T00:33:16.5122287Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-11-30T00:33:16.5122617Z error[E0723]: loops and conditional expressions are not stable in const fn
2019-11-30T00:33:16.5122899Z     --> src/libcore/num/mod.rs:851:16
2019-11-30T00:33:16.5123120Z      |
2019-11-30T00:33:16.5123427Z 238  | / macro_rules! int_impl {
2019-11-30T00:33:16.5123427Z 238  | / macro_rules! int_impl {
2019-11-30T00:33:16.5123832Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-11-30T00:33:16.5124229Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-11-30T00:33:16.5124602Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-11-30T00:33:16.5124857Z ...    |
2019-11-30T00:33:16.5125215Z 851  | |             if rhs == 0 || (self == Self::min_value() && rhs == -1) {
2019-11-30T00:33:16.5125962Z ...    |
2019-11-30T00:33:16.5126270Z 2394 | |     }
2019-11-30T00:33:16.5126579Z 2395 | | }
2019-11-30T00:33:16.5126579Z 2395 | | }
2019-11-30T00:33:16.5126894Z      | |_- in this expansion of `int_impl!`
2019-11-30T00:33:16.5127117Z ...
2019-11-30T00:33:16.5127499Z 2426 | /     int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
2019-11-30T00:33:16.5127867Z 2427 | |         170141183460469231731687303715884105727, "", "", 16,
2019-11-30T00:33:16.5128531Z 2428 | |         "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
2019-11-30T00:33:16.5128956Z 2429 | |         "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
2019-11-30T00:33:16.5129364Z ...    |
2019-11-30T00:33:16.5129728Z 2432 | |         "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
2019-11-30T00:33:16.5130108Z 2433 | |           0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }
2019-11-30T00:33:16.5130750Z      |
2019-11-30T00:33:16.5130750Z      |
2019-11-30T00:33:16.5131130Z      = note: for more information, see issue ***/issues/57563
2019-11-30T00:33:16.5131471Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-11-30T00:33:16.5131814Z error[E0723]: loops and conditional expressions are not stable in const fn
2019-11-30T00:33:16.5132080Z     --> src/libcore/num/mod.rs:851:16
2019-11-30T00:33:16.5132311Z      |
2019-11-30T00:33:16.5132624Z 238  | / macro_rules! int_impl {
2019-11-30T00:33:16.5132624Z 238  | / macro_rules! int_impl {
2019-11-30T00:33:16.5133016Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-11-30T00:33:16.5133425Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-11-30T00:33:16.5133787Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-11-30T00:33:16.5134048Z ...    |
2019-11-30T00:33:16.5134407Z 851  | |             if rhs == 0 || (self == Self::min_value() && rhs == -1) {
2019-11-30T00:33:16.5135115Z ...    |
2019-11-30T00:33:16.5135448Z 2394 | |     }
2019-11-30T00:33:16.5135741Z 2395 | | }
2019-11-30T00:33:16.5135741Z 2395 | | }
2019-11-30T00:33:16.5136092Z      | |_- in this expansion of `int_impl!`
2019-11-30T00:33:16.5136306Z ...
2019-11-30T00:33:16.5136689Z 2456 | /     int_impl! { isize, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "",
2019-11-30T00:33:16.5137114Z 2457 | |         12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
2019-11-30T00:33:16.5137521Z 2458 | |          "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2019-11-30T00:33:16.5137905Z 2459 | |          "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
2019-11-30T00:33:16.5138572Z 2460 | |          usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
2019-11-30T00:33:16.5139253Z      |
2019-11-30T00:33:16.5139253Z      |
2019-11-30T00:33:16.5139621Z      = note: for more information, see issue ***/issues/57563
2019-11-30T00:33:16.5139972Z      = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-11-30T00:33:17.5003056Z    Compiling backtrace-sys v0.1.32
2019-11-30T00:33:18.2686022Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2019-11-30T00:33:19.1798625Z    Compiling hashbrown v0.6.2
2019-11-30T00:33:19.4589355Z    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
---
2019-11-30T00:33:20.0153500Z   local time: Sat Nov 30 00:33:20 UTC 2019
2019-11-30T00:33:20.1014810Z   network time: Sat, 30 Nov 2019 00:33:20 GMT
2019-11-30T00:33:20.1017060Z == end clock drift check ==
2019-11-30T00:33:21.5254916Z 
2019-11-30T00:33:21.5356707Z ##[error]Bash exited with code '1'.
2019-11-30T00:33:21.5385706Z ##[section]Starting: Checkout
2019-11-30T00:33:21.5387406Z ==============================================================================
2019-11-30T00:33:21.5387477Z Task         : Get sources
2019-11-30T00:33:21.5387543Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
