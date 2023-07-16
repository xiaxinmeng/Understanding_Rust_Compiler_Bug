plain
2019-12-16T10:17:35.9926673Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-16T10:17:36.0093279Z ##[command]git config gc.auto 0
2019-12-16T10:17:36.0166331Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-16T10:17:36.0209335Z ##[command]git config --get-all http.proxy
2019-12-16T10:17:36.9978377Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66498/merge:refs/remotes/pull/66498/merge
---
2019-12-16T10:43:37.1467741Z    Compiling autocfg v0.1.6
2019-12-16T10:43:38.2062119Z    Compiling std v0.0.0 (/checkout/src/libstd)
2019-12-16T10:43:38.5966521Z    Compiling cmake v0.1.38
2019-12-16T10:43:40.6239513Z    Compiling compiler_builtins v0.1.22
2019-12-16T10:43:41.4333108Z error: `num::<impl i8>::to_ne_bytes` is not yet stable as a const fn
2019-12-16T10:43:41.4334686Z     --> src/libcore/num/mod.rs:2217:17
2019-12-16T10:43:41.4335896Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:41.4335896Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:41.4336475Z 249  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-16T10:43:41.4337052Z 250  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-16T10:43:41.4337785Z 251  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-16T10:43:41.4338250Z ...    |
2019-12-16T10:43:41.4338773Z 2217 | |                 self.to_be().to_ne_bytes()
2019-12-16T10:43:41.4339698Z ...    |
2019-12-16T10:43:41.4340182Z 2406 | |     }
2019-12-16T10:43:41.4340825Z 2407 | | }
2019-12-16T10:43:41.4341550Z      | |_- in this expansion of `int_impl!`
2019-12-16T10:43:41.4341550Z      | |_- in this expansion of `int_impl!`
2019-12-16T10:43:41.4342011Z ...
2019-12-16T10:43:41.4342631Z 2411 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-12-16T10:43:41.4343215Z 2412 | |     "[0x12]", "[0x12]", "", "" }
2019-12-16T10:43:41.4344557Z      |
2019-12-16T10:43:41.4345066Z      = help: add `#![feature(const_int_conversion)]` to the crate attributes to enable
2019-12-16T10:43:41.4347798Z 
2019-12-16T10:43:41.4347798Z 
2019-12-16T10:43:41.5009542Z error: `num::<impl i16>::to_ne_bytes` is not yet stable as a const fn
2019-12-16T10:43:41.5010340Z     --> src/libcore/num/mod.rs:2217:17
2019-12-16T10:43:41.5011372Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:41.5011372Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:41.5011939Z 249  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-16T10:43:41.5012534Z 250  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-16T10:43:41.5013059Z 251  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-16T10:43:41.5013543Z ...    |
2019-12-16T10:43:41.5014234Z 2217 | |                 self.to_be().to_ne_bytes()
2019-12-16T10:43:41.5016296Z ...    |
2019-12-16T10:43:41.5016922Z 2406 | |     }
2019-12-16T10:43:41.5017426Z 2407 | | }
2019-12-16T10:43:41.5017891Z      | |_- in this expansion of `int_impl!`
2019-12-16T10:43:41.5017891Z      | |_- in this expansion of `int_impl!`
2019-12-16T10:43:41.5018265Z ...
2019-12-16T10:43:41.5019265Z 2417 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
2019-12-16T10:43:41.5019939Z 2418 | |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
2019-12-16T10:43:41.5020772Z      |
2019-12-16T10:43:41.5021192Z      = help: add `#![feature(const_int_conversion)]` to the crate attributes to enable
2019-12-16T10:43:41.5021348Z 
2019-12-16T10:43:41.5021348Z 
2019-12-16T10:43:41.5769300Z error: `num::<impl i32>::to_ne_bytes` is not yet stable as a const fn
2019-12-16T10:43:41.5769622Z     --> src/libcore/num/mod.rs:2217:17
2019-12-16T10:43:41.5770135Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:41.5770135Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:41.5770509Z 249  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-16T10:43:41.5770860Z 250  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-16T10:43:41.5771183Z 251  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-16T10:43:41.5771411Z ...    |
2019-12-16T10:43:41.5771697Z 2217 | |                 self.to_be().to_ne_bytes()
2019-12-16T10:43:41.5772234Z ...    |
2019-12-16T10:43:41.5772501Z 2406 | |     }
2019-12-16T10:43:41.5772755Z 2407 | | }
2019-12-16T10:43:41.5773025Z      | |_- in this expansion of `int_impl!`
2019-12-16T10:43:41.5773025Z      | |_- in this expansion of `int_impl!`
2019-12-16T10:43:41.5773223Z ...
2019-12-16T10:43:41.5773567Z 2423 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
2019-12-16T10:43:41.5774056Z 2424 | |     "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
2019-12-16T10:43:41.5774419Z 2425 | |     "[0x12, 0x34, 0x56, 0x78]", "", "" }
2019-12-16T10:43:41.5775090Z      |
2019-12-16T10:43:41.5775386Z      = help: add `#![feature(const_int_conversion)]` to the crate attributes to enable
2019-12-16T10:43:41.5778202Z 
2019-12-16T10:43:41.5778202Z 
2019-12-16T10:43:41.6491912Z error: `num::<impl i64>::to_ne_bytes` is not yet stable as a const fn
2019-12-16T10:43:41.6492397Z     --> src/libcore/num/mod.rs:2217:17
2019-12-16T10:43:41.6492861Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:41.6492861Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:41.6493165Z 249  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-16T10:43:41.6493486Z 250  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-16T10:43:41.6493760Z 251  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-16T10:43:41.6493971Z ...    |
2019-12-16T10:43:41.6494231Z 2217 | |                 self.to_be().to_ne_bytes()
2019-12-16T10:43:41.6494698Z ...    |
2019-12-16T10:43:41.6494918Z 2406 | |     }
2019-12-16T10:43:41.6495136Z 2407 | | }
2019-12-16T10:43:41.6495399Z      | |_- in this expansion of `int_impl!`
2019-12-16T10:43:41.6495399Z      | |_- in this expansion of `int_impl!`
2019-12-16T10:43:41.6495741Z ...
2019-12-16T10:43:41.6496220Z 2430 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
2019-12-16T10:43:41.6496774Z 2431 | |     "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
2019-12-16T10:43:41.6497114Z 2432 | |     "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2019-12-16T10:43:41.6498276Z 2433 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
2019-12-16T10:43:41.6502640Z      |
2019-12-16T10:43:41.6506943Z      = help: add `#![feature(const_int_conversion)]` to the crate attributes to enable
2019-12-16T10:43:41.6507015Z 
2019-12-16T10:43:41.6507015Z 
2019-12-16T10:43:41.7375717Z error: `num::<impl i128>::to_ne_bytes` is not yet stable as a const fn
2019-12-16T10:43:41.7376105Z     --> src/libcore/num/mod.rs:2217:17
2019-12-16T10:43:41.7376612Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:41.7376612Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:41.7377169Z 249  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-16T10:43:41.7377584Z 250  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-16T10:43:41.7378083Z 251  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-16T10:43:41.7378369Z ...    |
2019-12-16T10:43:41.7378689Z 2217 | |                 self.to_be().to_ne_bytes()
2019-12-16T10:43:41.7379271Z ...    |
2019-12-16T10:43:41.7379555Z 2406 | |     }
2019-12-16T10:43:41.7380162Z 2407 | | }
2019-12-16T10:43:41.7380594Z      | |_- in this expansion of `int_impl!`
2019-12-16T10:43:41.7380594Z      | |_- in this expansion of `int_impl!`
2019-12-16T10:43:41.7380783Z ...
2019-12-16T10:43:41.7381127Z 2438 | /     int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
2019-12-16T10:43:41.7381437Z 2439 | |     170141183460469231731687303715884105727, "", "", 16,
2019-12-16T10:43:41.7381778Z 2440 | |     "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
2019-12-16T10:43:41.7382128Z 2441 | |     "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
2019-12-16T10:43:41.7382341Z ...    |
2019-12-16T10:43:41.7382673Z 2444 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
2019-12-16T10:43:41.7382986Z 2445 | |       0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }
2019-12-16T10:43:41.7383512Z      |
2019-12-16T10:43:41.7383780Z      = help: add `#![feature(const_int_conversion)]` to the crate attributes to enable
2019-12-16T10:43:41.7387815Z 
2019-12-16T10:43:41.7387815Z 
2019-12-16T10:43:41.8053687Z error: `num::<impl isize>::to_ne_bytes` is not yet stable as a const fn
2019-12-16T10:43:41.8054071Z     --> src/libcore/num/mod.rs:2217:17
2019-12-16T10:43:41.8055097Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:41.8055097Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:41.8055501Z 249  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-16T10:43:41.8055889Z 250  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-16T10:43:41.8056204Z 251  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-16T10:43:41.8056432Z ...    |
2019-12-16T10:43:41.8056867Z 2217 | |                 self.to_be().to_ne_bytes()
2019-12-16T10:43:41.8057401Z ...    |
2019-12-16T10:43:41.8057687Z 2406 | |     }
2019-12-16T10:43:41.8057949Z 2407 | | }
2019-12-16T10:43:41.8058251Z      | |_- in this expansion of `int_impl!`
2019-12-16T10:43:41.8058251Z      | |_- in this expansion of `int_impl!`
2019-12-16T10:43:41.8058442Z ...
2019-12-16T10:43:41.8058789Z 2468 | /     int_impl! { isize, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "",
2019-12-16T10:43:41.8059187Z 2469 | |     12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
2019-12-16T10:43:41.8059546Z 2470 | |      "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2019-12-16T10:43:41.8059871Z 2471 | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
2019-12-16T10:43:41.8060765Z 2472 | |      usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
2019-12-16T10:43:41.8061403Z      |
2019-12-16T10:43:41.8061723Z      = help: add `#![feature(const_int_conversion)]` to the crate attributes to enable
2019-12-16T10:43:41.8066753Z 
2019-12-16T10:43:41.8066753Z 
2019-12-16T10:43:41.8861515Z error: `num::<impl i8>::to_ne_bytes` is not yet stable as a const fn
2019-12-16T10:43:41.8861873Z     --> src/libcore/num/mod.rs:2241:17
2019-12-16T10:43:41.8862580Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:41.8862580Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:41.8862948Z 249  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-16T10:43:41.8863656Z 250  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-16T10:43:41.8864039Z 251  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-16T10:43:41.8864606Z 2241 | |                 self.to_le().to_ne_bytes()
2019-12-16T10:43:41.8865098Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-16T10:43:41.8865342Z ...    |
2019-12-16T10:43:41.8865644Z 2406 | |     }
2019-12-16T10:43:41.8865644Z 2406 | |     }
2019-12-16T10:43:41.8866071Z 2407 | | }
2019-12-16T10:43:41.8866390Z      | |_- in this expansion of `int_impl!`
2019-12-16T10:43:41.8866617Z ...
2019-12-16T10:43:41.8866994Z 2411 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-12-16T10:43:41.8867334Z 2412 | |     "[0x12]", "[0x12]", "", "" }
2019-12-16T10:43:41.8867884Z      |
2019-12-16T10:43:41.8868227Z      = help: add `#![feature(const_int_conversion)]` to the crate attributes to enable
2019-12-16T10:43:41.8916050Z 
2019-12-16T10:43:41.8916050Z 
2019-12-16T10:43:41.9494062Z error: `num::<impl i16>::to_ne_bytes` is not yet stable as a const fn
2019-12-16T10:43:41.9494411Z     --> src/libcore/num/mod.rs:2241:17
2019-12-16T10:43:41.9495126Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:41.9495126Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:41.9495500Z 249  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-16T10:43:41.9495910Z 250  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-16T10:43:41.9496269Z 251  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-16T10:43:41.9497219Z 2241 | |                 self.to_le().to_ne_bytes()
2019-12-16T10:43:41.9497596Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-16T10:43:41.9497861Z ...    |
2019-12-16T10:43:41.9498144Z 2406 | |     }
2019-12-16T10:43:41.9498144Z 2406 | |     }
2019-12-16T10:43:41.9498588Z 2407 | | }
2019-12-16T10:43:41.9498917Z      | |_- in this expansion of `int_impl!`
2019-12-16T10:43:41.9499278Z ...
2019-12-16T10:43:41.9499701Z 2417 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
2019-12-16T10:43:41.9500063Z 2418 | |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
2019-12-16T10:43:41.9500662Z      |
2019-12-16T10:43:41.9500966Z      = help: add `#![feature(const_int_conversion)]` to the crate attributes to enable
2019-12-16T10:43:41.9505362Z 
2019-12-16T10:43:41.9505362Z 
2019-12-16T10:43:42.0292252Z error: `num::<impl i32>::to_ne_bytes` is not yet stable as a const fn
2019-12-16T10:43:42.0292657Z     --> src/libcore/num/mod.rs:2241:17
2019-12-16T10:43:42.0293371Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:42.0293371Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:42.0293732Z 249  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-16T10:43:42.0294130Z 250  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-16T10:43:42.0294466Z 251  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-16T10:43:42.0295060Z 2241 | |                 self.to_le().to_ne_bytes()
2019-12-16T10:43:42.0295382Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-16T10:43:42.0295617Z ...    |
2019-12-16T10:43:42.0295889Z 2406 | |     }
2019-12-16T10:43:42.0295889Z 2406 | |     }
2019-12-16T10:43:42.0296177Z 2407 | | }
2019-12-16T10:43:42.0296474Z      | |_- in this expansion of `int_impl!`
2019-12-16T10:43:42.0296680Z ...
2019-12-16T10:43:42.0297075Z 2423 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
2019-12-16T10:43:42.0297442Z 2424 | |     "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
2019-12-16T10:43:42.0297758Z 2425 | |     "[0x12, 0x34, 0x56, 0x78]", "", "" }
2019-12-16T10:43:42.0298312Z      |
2019-12-16T10:43:42.0298608Z      = help: add `#![feature(const_int_conversion)]` to the crate attributes to enable
2019-12-16T10:43:42.0302821Z 
2019-12-16T10:43:42.0302821Z 
2019-12-16T10:43:42.0975193Z error: `num::<impl i64>::to_ne_bytes` is not yet stable as a const fn
2019-12-16T10:43:42.0975596Z     --> src/libcore/num/mod.rs:2241:17
2019-12-16T10:43:42.0976102Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:42.0976102Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:42.0976483Z 249  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-16T10:43:42.0976852Z 250  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-16T10:43:42.0977311Z 251  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-16T10:43:42.0977888Z 2241 | |                 self.to_le().to_ne_bytes()
2019-12-16T10:43:42.0978213Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-16T10:43:42.0978448Z ...    |
2019-12-16T10:43:42.0978721Z 2406 | |     }
2019-12-16T10:43:42.0978721Z 2406 | |     }
2019-12-16T10:43:42.0979010Z 2407 | | }
2019-12-16T10:43:42.0979307Z      | |_- in this expansion of `int_impl!`
2019-12-16T10:43:42.0979513Z ...
2019-12-16T10:43:42.0979902Z 2430 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
2019-12-16T10:43:42.0980278Z 2431 | |     "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
2019-12-16T10:43:42.0980835Z 2432 | |     "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2019-12-16T10:43:42.0981216Z 2433 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
2019-12-16T10:43:42.0981826Z      |
2019-12-16T10:43:42.0982148Z      = help: add `#![feature(const_int_conversion)]` to the crate attributes to enable
2019-12-16T10:43:42.0986000Z 
2019-12-16T10:43:42.0986000Z 
2019-12-16T10:43:42.1597839Z error: `num::<impl i128>::to_ne_bytes` is not yet stable as a const fn
2019-12-16T10:43:42.1598221Z     --> src/libcore/num/mod.rs:2241:17
2019-12-16T10:43:42.1598755Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:42.1598755Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:42.1599307Z 249  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-16T10:43:42.1599897Z 250  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-16T10:43:42.1600301Z 251  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-16T10:43:42.1600864Z 2241 | |                 self.to_le().to_ne_bytes()
2019-12-16T10:43:42.1601373Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-16T10:43:42.1601610Z ...    |
2019-12-16T10:43:42.1602045Z 2406 | |     }
2019-12-16T10:43:42.1602045Z 2406 | |     }
2019-12-16T10:43:42.1602333Z 2407 | | }
2019-12-16T10:43:42.1602640Z      | |_- in this expansion of `int_impl!`
2019-12-16T10:43:42.1602860Z ...
2019-12-16T10:43:42.1603228Z 2438 | /     int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
2019-12-16T10:43:42.1603573Z 2439 | |     170141183460469231731687303715884105727, "", "", 16,
2019-12-16T10:43:42.1603987Z 2440 | |     "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
2019-12-16T10:43:42.1604382Z 2441 | |     "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
2019-12-16T10:43:42.1604792Z ...    |
2019-12-16T10:43:42.1605114Z 2444 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
2019-12-16T10:43:42.1605454Z 2445 | |       0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }
2019-12-16T10:43:42.1606026Z      |
2019-12-16T10:43:42.1606374Z      = help: add `#![feature(const_int_conversion)]` to the crate attributes to enable
2019-12-16T10:43:42.1606425Z 
2019-12-16T10:43:42.1606425Z 
2019-12-16T10:43:42.1982955Z error: `num::<impl isize>::to_ne_bytes` is not yet stable as a const fn
2019-12-16T10:43:42.1983336Z     --> src/libcore/num/mod.rs:2241:17
2019-12-16T10:43:42.1983861Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:42.1983861Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:42.1984416Z 249  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-16T10:43:42.1984784Z 250  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-16T10:43:42.1985119Z 251  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-16T10:43:42.1985893Z 2241 | |                 self.to_le().to_ne_bytes()
2019-12-16T10:43:42.1986196Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-16T10:43:42.1986448Z ...    |
2019-12-16T10:43:42.1986719Z 2406 | |     }
2019-12-16T10:43:42.1986719Z 2406 | |     }
2019-12-16T10:43:42.1987002Z 2407 | | }
2019-12-16T10:43:42.1987421Z      | |_- in this expansion of `int_impl!`
2019-12-16T10:43:42.1987620Z ...
2019-12-16T10:43:42.1988002Z 2468 | /     int_impl! { isize, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "",
2019-12-16T10:43:42.1988377Z 2469 | |     12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
2019-12-16T10:43:42.1988741Z 2470 | |      "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2019-12-16T10:43:42.1989095Z 2471 | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
2019-12-16T10:43:42.1989444Z 2472 | |      usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
2019-12-16T10:43:42.1990142Z      |
2019-12-16T10:43:42.1990428Z      = help: add `#![feature(const_int_conversion)]` to the crate attributes to enable
2019-12-16T10:43:42.1994949Z 
2019-12-16T10:43:42.2652789Z error: `intrinsics::transmute` is not yet stable as a const fn
2019-12-16T10:43:42.2652789Z error: `intrinsics::transmute` is not yet stable as a const fn
2019-12-16T10:43:42.2653162Z     --> src/libcore/num/mod.rs:2282:26
2019-12-16T10:43:42.2653397Z      |
2019-12-16T10:43:42.2653698Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:42.2654154Z 249  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-16T10:43:42.2654547Z 250  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-16T10:43:42.2655055Z 251  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-16T10:43:42.2655609Z 2282 | |                 unsafe { mem::transmute(self) }
2019-12-16T10:43:42.2655907Z      | |                          ^^^^^^^^^^^^^^^^^^^^
2019-12-16T10:43:42.2656146Z ...    |
2019-12-16T10:43:42.2656611Z 2406 | |     }
2019-12-16T10:43:42.2656611Z 2406 | |     }
2019-12-16T10:43:42.2656925Z 2407 | | }
2019-12-16T10:43:42.2657207Z      | |_- in this expansion of `int_impl!`
2019-12-16T10:43:42.2657421Z ...
2019-12-16T10:43:42.2657770Z 2411 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-12-16T10:43:42.2658084Z 2412 | |     "[0x12]", "[0x12]", "", "" }
2019-12-16T10:43:42.2658734Z      |
2019-12-16T10:43:42.2659043Z      = help: add `#![feature(const_transmute)]` to the crate attributes to enable
2019-12-16T10:43:42.2663263Z 
2019-12-16T10:43:42.3372343Z error: `intrinsics::transmute` is not yet stable as a const fn
2019-12-16T10:43:42.3372343Z error: `intrinsics::transmute` is not yet stable as a const fn
2019-12-16T10:43:42.3375558Z     --> src/libcore/num/mod.rs:2282:26
2019-12-16T10:43:42.3375803Z      |
2019-12-16T10:43:42.3376128Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:42.3376514Z 249  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-16T10:43:42.3377028Z 250  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-16T10:43:42.3377393Z 251  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-16T10:43:42.3378077Z 2282 | |                 unsafe { mem::transmute(self) }
2019-12-16T10:43:42.3378396Z      | |                          ^^^^^^^^^^^^^^^^^^^^
2019-12-16T10:43:42.3379040Z ...    |
2019-12-16T10:43:42.3379323Z 2406 | |     }
2019-12-16T10:43:42.3379323Z 2406 | |     }
2019-12-16T10:43:42.3379581Z 2407 | | }
2019-12-16T10:43:42.3379870Z      | |_- in this expansion of `int_impl!`
2019-12-16T10:43:42.3380057Z ...
2019-12-16T10:43:42.3380403Z 2417 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
2019-12-16T10:43:42.3380737Z 2418 | |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
2019-12-16T10:43:42.3381445Z      |
2019-12-16T10:43:42.3381882Z      = help: add `#![feature(const_transmute)]` to the crate attributes to enable
2019-12-16T10:43:42.3385435Z 
2019-12-16T10:43:42.4107704Z error: `intrinsics::transmute` is not yet stable as a const fn
2019-12-16T10:43:42.4107704Z error: `intrinsics::transmute` is not yet stable as a const fn
2019-12-16T10:43:42.4108054Z     --> src/libcore/num/mod.rs:2282:26
2019-12-16T10:43:42.4108577Z      |
2019-12-16T10:43:42.4108911Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:42.4109286Z 249  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-16T10:43:42.4109962Z 250  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-16T10:43:42.4110334Z 251  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-16T10:43:42.4110928Z 2282 | |                 unsafe { mem::transmute(self) }
2019-12-16T10:43:42.4111245Z      | |                          ^^^^^^^^^^^^^^^^^^^^
2019-12-16T10:43:42.4111655Z ...    |
2019-12-16T10:43:42.4111927Z 2406 | |     }
2019-12-16T10:43:42.4111927Z 2406 | |     }
2019-12-16T10:43:42.4112256Z 2407 | | }
2019-12-16T10:43:42.4112584Z      | |_- in this expansion of `int_impl!`
2019-12-16T10:43:42.4112785Z ...
2019-12-16T10:43:42.4113152Z 2423 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
2019-12-16T10:43:42.4113690Z 2424 | |     "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
2019-12-16T10:43:42.4113997Z 2425 | |     "[0x12, 0x34, 0x56, 0x78]", "", "" }
2019-12-16T10:43:42.4114545Z      |
2019-12-16T10:43:42.4116670Z      = help: add `#![feature(const_transmute)]` to the crate attributes to enable
2019-12-16T10:43:42.4167619Z 
2019-12-16T10:43:42.4895126Z error: `intrinsics::transmute` is not yet stable as a const fn
2019-12-16T10:43:42.4895126Z error: `intrinsics::transmute` is not yet stable as a const fn
2019-12-16T10:43:42.4895473Z     --> src/libcore/num/mod.rs:2282:26
2019-12-16T10:43:42.4895718Z      |
2019-12-16T10:43:42.4896015Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:42.4896375Z 249  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-16T10:43:42.4896958Z 250  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-16T10:43:42.4897330Z 251  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-16T10:43:42.4897900Z 2282 | |                 unsafe { mem::transmute(self) }
2019-12-16T10:43:42.4898205Z      | |                          ^^^^^^^^^^^^^^^^^^^^
2019-12-16T10:43:42.4898457Z ...    |
2019-12-16T10:43:42.4898729Z 2406 | |     }
2019-12-16T10:43:42.4898729Z 2406 | |     }
2019-12-16T10:43:42.4899123Z 2407 | | }
2019-12-16T10:43:42.4899450Z      | |_- in this expansion of `int_impl!`
2019-12-16T10:43:42.4899648Z ...
2019-12-16T10:43:42.4900008Z 2430 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
2019-12-16T10:43:42.4900393Z 2431 | |     "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
2019-12-16T10:43:42.4900755Z 2432 | |     "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2019-12-16T10:43:42.4901130Z 2433 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
2019-12-16T10:43:42.4901704Z      |
2019-12-16T10:43:42.4902018Z      = help: add `#![feature(const_transmute)]` to the crate attributes to enable
2019-12-16T10:43:42.4906314Z 
2019-12-16T10:43:42.5550622Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2019-12-16T10:43:42.5550622Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2019-12-16T10:43:42.5736033Z error: `intrinsics::transmute` is not yet stable as a const fn
2019-12-16T10:43:42.5736373Z     --> src/libcore/num/mod.rs:2282:26
2019-12-16T10:43:42.5736628Z      |
2019-12-16T10:43:42.5736949Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:42.5737341Z 249  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-16T10:43:42.5737728Z 250  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-16T10:43:42.5738055Z 251  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-16T10:43:42.5738618Z 2282 | |                 unsafe { mem::transmute(self) }
2019-12-16T10:43:42.5739103Z      | |                          ^^^^^^^^^^^^^^^^^^^^
2019-12-16T10:43:42.5739379Z ...    |
2019-12-16T10:43:42.5739670Z 2406 | |     }
2019-12-16T10:43:42.5739670Z 2406 | |     }
2019-12-16T10:43:42.5739935Z 2407 | | }
2019-12-16T10:43:42.5740239Z      | |_- in this expansion of `int_impl!`
2019-12-16T10:43:42.5740434Z ...
2019-12-16T10:43:42.5740786Z 2438 | /     int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
2019-12-16T10:43:42.5741261Z 2439 | |     170141183460469231731687303715884105727, "", "", 16,
2019-12-16T10:43:42.5741651Z 2440 | |     "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
2019-12-16T10:43:42.5742016Z 2441 | |     "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
2019-12-16T10:43:42.5742271Z ...    |
2019-12-16T10:43:42.5742599Z 2444 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
2019-12-16T10:43:42.5742957Z 2445 | |       0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }
2019-12-16T10:43:42.5743539Z      |
2019-12-16T10:43:42.5743851Z      = help: add `#![feature(const_transmute)]` to the crate attributes to enable
2019-12-16T10:43:42.5748162Z 
2019-12-16T10:43:42.6452452Z error: `intrinsics::transmute` is not yet stable as a const fn
2019-12-16T10:43:42.6452452Z error: `intrinsics::transmute` is not yet stable as a const fn
2019-12-16T10:43:42.6452801Z     --> src/libcore/num/mod.rs:2282:26
2019-12-16T10:43:42.6453001Z      |
2019-12-16T10:43:42.6453270Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:42.6453625Z 249  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-16T10:43:42.6454154Z 250  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-16T10:43:42.6454466Z 251  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-16T10:43:42.6454956Z 2282 | |                 unsafe { mem::transmute(self) }
2019-12-16T10:43:42.6455249Z      | |                          ^^^^^^^^^^^^^^^^^^^^
2019-12-16T10:43:42.6455468Z ...    |
2019-12-16T10:43:42.6455710Z 2406 | |     }
2019-12-16T10:43:42.6455710Z 2406 | |     }
2019-12-16T10:43:42.6456131Z 2407 | | }
2019-12-16T10:43:42.6456438Z      | |_- in this expansion of `int_impl!`
2019-12-16T10:43:42.6456614Z ...
2019-12-16T10:43:42.6456955Z 2468 | /     int_impl! { isize, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "",
2019-12-16T10:43:42.6457287Z 2469 | |     12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
2019-12-16T10:43:42.6457633Z 2470 | |      "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2019-12-16T10:43:42.6458066Z 2471 | |      "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
2019-12-16T10:43:42.6458373Z 2472 | |      usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
2019-12-16T10:43:42.6458902Z      |
2019-12-16T10:43:42.6459297Z      = help: add `#![feature(const_transmute)]` to the crate attributes to enable
2019-12-16T10:43:42.6463592Z 
2019-12-16T10:43:42.6463592Z 
2019-12-16T10:43:42.7213346Z error: `num::<impl i8>::from_ne_bytes` is not yet stable as a const fn
2019-12-16T10:43:42.7213678Z     --> src/libcore/num/mod.rs:2318:31
2019-12-16T10:43:42.7214151Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:42.7214151Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:42.7214486Z 249  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-16T10:43:42.7214805Z 250  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-16T10:43:42.7215084Z 251  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-16T10:43:42.7215323Z ...    |
2019-12-16T10:43:42.7215790Z 2318 | |                 Self::from_be(Self::from_ne_bytes(bytes))
2019-12-16T10:43:42.7216874Z ...    |
2019-12-16T10:43:42.7217307Z 2406 | |     }
2019-12-16T10:43:42.7217579Z 2407 | | }
2019-12-16T10:43:42.7218049Z      | |_- in this expansion of `int_impl!`
2019-12-16T10:43:42.7218049Z      | |_- in this expansion of `int_impl!`
2019-12-16T10:43:42.7218253Z ...
2019-12-16T10:43:42.7218802Z 2411 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-12-16T10:43:42.7219157Z 2412 | |     "[0x12]", "[0x12]", "", "" }
2019-12-16T10:43:42.7220360Z      |
2019-12-16T10:43:42.7220636Z      = help: add `#![feature(const_int_conversion)]` to the crate attributes to enable
2019-12-16T10:43:42.7224953Z 
2019-12-16T10:43:42.7224953Z 
2019-12-16T10:43:42.7933539Z error: `num::<impl i16>::from_ne_bytes` is not yet stable as a const fn
2019-12-16T10:43:42.7934253Z     --> src/libcore/num/mod.rs:2318:31
2019-12-16T10:43:42.7934774Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:42.7934774Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:42.7935114Z 249  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-16T10:43:42.7935470Z 250  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-16T10:43:42.7935770Z 251  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-16T10:43:42.7936308Z ...    |
2019-12-16T10:43:42.7936637Z 2318 | |                 Self::from_be(Self::from_ne_bytes(bytes))
2019-12-16T10:43:42.7937140Z ...    |
2019-12-16T10:43:42.7937376Z 2406 | |     }
2019-12-16T10:43:42.7937611Z 2407 | | }
2019-12-16T10:43:42.7937885Z      | |_- in this expansion of `int_impl!`
2019-12-16T10:43:42.7937885Z      | |_- in this expansion of `int_impl!`
2019-12-16T10:43:42.7938058Z ...
2019-12-16T10:43:42.7938381Z 2417 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
2019-12-16T10:43:42.7938711Z 2418 | |     "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
2019-12-16T10:43:42.7939189Z      |
2019-12-16T10:43:42.7939470Z      = help: add `#![feature(const_int_conversion)]` to the crate attributes to enable
2019-12-16T10:43:42.7943577Z 
2019-12-16T10:43:42.7943577Z 
2019-12-16T10:43:42.8732830Z error: `num::<impl i32>::from_ne_bytes` is not yet stable as a const fn
2019-12-16T10:43:42.8733170Z     --> src/libcore/num/mod.rs:2318:31
2019-12-16T10:43:42.8733651Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:42.8733651Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:42.8734142Z 249  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-16T10:43:42.8734520Z 250  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-16T10:43:42.8734830Z 251  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-16T10:43:42.8735043Z ...    |
2019-12-16T10:43:42.8735341Z 2318 | |                 Self::from_be(Self::from_ne_bytes(bytes))
2019-12-16T10:43:42.8735953Z ...    |
2019-12-16T10:43:42.8736389Z 2406 | |     }
2019-12-16T10:43:42.8736629Z 2407 | | }
2019-12-16T10:43:42.8737141Z      | |_- in this expansion of `int_impl!`
2019-12-16T10:43:42.8737141Z      | |_- in this expansion of `int_impl!`
2019-12-16T10:43:42.8737338Z ...
2019-12-16T10:43:42.8737703Z 2423 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
2019-12-16T10:43:42.8738095Z 2424 | |     "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
2019-12-16T10:43:42.8738421Z 2425 | |     "[0x12, 0x34, 0x56, 0x78]", "", "" }
2019-12-16T10:43:42.8738974Z      |
2019-12-16T10:43:42.8739270Z      = help: add `#![feature(const_int_conversion)]` to the crate attributes to enable
2019-12-16T10:43:42.8743459Z 
2019-12-16T10:43:42.8743459Z 
2019-12-16T10:43:42.9493270Z error: `num::<impl i64>::from_ne_bytes` is not yet stable as a const fn
2019-12-16T10:43:42.9493583Z     --> src/libcore/num/mod.rs:2318:31
2019-12-16T10:43:42.9494150Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:42.9494150Z 248  | / macro_rules! int_impl {
2019-12-16T10:43:42.9494509Z 249  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-16T10:43:42.9495081Z 250  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-16T10:43:42.9495416Z 251  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-16T10:43:42.9495679Z ...    |
2019-12-16T10:43:42.9496017Z 2318 | |                 Self::from_be(Self::from_ne_bytes(bytes))
2019-12-16T10:43:42.9496822Z ...    |
2019-12-16T10:43:42.9497093Z 2406 | |     }
2019-12-16T10:43:42.9497362Z 2407 | | }
2019-12-16T10:43:42.9497679Z      | |_- in this expansion of `int_impl!`
2019-12-16T10:43:42.9497679Z      | |_- in this expansion of `int_impl!`
2019-12-16T10:43:42.9497878Z ...
2019-12-16T10:43:42.9498239Z 2430 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
2019-12-16T10:43:42.9498761Z 2431 | |     "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
2019-12-16T10:43:42.9499131Z 2432 | |     "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2019-12-16T10:43:42.9499496Z 2433 | |     "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
2019-12-16T10:43:42.9500065Z      |
---
2019-12-16T10:43:52.5022880Z   local time: Mon Dec 16 10:43:52 UTC 2019
2019-12-16T10:43:52.7744975Z   network time: Mon, 16 Dec 2019 10:43:52 GMT
2019-12-16T10:43:52.7750277Z == end clock drift check ==
2019-12-16T10:43:55.9986058Z 
2019-12-16T10:43:56.0078320Z ##[error]Bash exited with code '1'.
2019-12-16T10:43:56.0107097Z ##[section]Starting: Checkout
2019-12-16T10:43:56.0108583Z ==============================================================================
2019-12-16T10:43:56.0108626Z Task         : Get sources
2019-12-16T10:43:56.0108663Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
