plain
2019-09-28T20:07:28.4593097Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-28T20:07:28.4779587Z ##[command]git config gc.auto 0
2019-09-28T20:07:28.4854180Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-28T20:07:28.4905972Z ##[command]git config --get-all http.proxy
2019-09-28T20:07:28.5052974Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/61835/merge:refs/remotes/pull/61835/merge
---
2019-09-28T20:39:21.9570449Z    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
2019-09-28T20:39:22.1623559Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-28T20:39:22.1624169Z     --> src/libcore/num/mod.rs:1115:17
2019-09-28T20:39:22.1624556Z      |
2019-09-28T20:39:22.1624885Z 238  | / macro_rules! int_impl {
2019-09-28T20:39:22.1625279Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-28T20:39:22.1625682Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-28T20:39:22.1626034Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-28T20:39:22.1626663Z 1115 | |                 intrinsics::wrapping_add(self, rhs)
2019-09-28T20:39:22.1627001Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-28T20:39:22.1627272Z ...    |
2019-09-28T20:39:22.1627562Z 2224 | |     }
2019-09-28T20:39:22.1627562Z 2224 | |     }
2019-09-28T20:39:22.1627853Z 2225 | | }
2019-09-28T20:39:22.1628178Z      | |_- in this expansion of `int_impl!`
2019-09-28T20:39:22.1628394Z ...
2019-09-28T20:39:22.1629075Z 2229 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-09-28T20:39:22.1629429Z 2230 | |         "[0x12]", "[0x12]", "", "" }
2019-09-28T20:39:22.1634974Z 
2019-09-28T20:39:22.1950574Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-28T20:39:22.1950939Z     --> src/libcore/num/mod.rs:1115:17
2019-09-28T20:39:22.1951209Z      |
2019-09-28T20:39:22.1951209Z      |
2019-09-28T20:39:22.1951523Z 238  | / macro_rules! int_impl {
2019-09-28T20:39:22.1952648Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-28T20:39:22.1953383Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-28T20:39:22.1953795Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-28T20:39:22.1954413Z 1115 | |                 intrinsics::wrapping_add(self, rhs)
2019-09-28T20:39:22.1954742Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-28T20:39:22.1955028Z ...    |
2019-09-28T20:39:22.1955318Z 2224 | |     }
2019-09-28T20:39:22.1955318Z 2224 | |     }
2019-09-28T20:39:22.1955616Z 2225 | | }
2019-09-28T20:39:22.1955944Z      | |_- in this expansion of `int_impl!`
2019-09-28T20:39:22.1956157Z ...
2019-09-28T20:39:22.1956544Z 2235 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
2019-09-28T20:39:22.1956923Z 2236 | |         "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
2019-09-28T20:39:22.1962486Z 
2019-09-28T20:39:22.2266115Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-28T20:39:22.2266491Z     --> src/libcore/num/mod.rs:1115:17
2019-09-28T20:39:22.2266735Z      |
2019-09-28T20:39:22.2266735Z      |
2019-09-28T20:39:22.2267043Z 238  | / macro_rules! int_impl {
2019-09-28T20:39:22.2267442Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-28T20:39:22.2268089Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-28T20:39:22.2268464Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-28T20:39:22.2269392Z 1115 | |                 intrinsics::wrapping_add(self, rhs)
2019-09-28T20:39:22.2270517Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-28T20:39:22.2270781Z ...    |
2019-09-28T20:39:22.2271066Z 2224 | |     }
2019-09-28T20:39:22.2271066Z 2224 | |     }
2019-09-28T20:39:22.2271392Z 2225 | | }
2019-09-28T20:39:22.2271699Z      | |_- in this expansion of `int_impl!`
2019-09-28T20:39:22.2271912Z ...
2019-09-28T20:39:22.2272441Z 2241 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
2019-09-28T20:39:22.2272869Z 2242 | |         "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
2019-09-28T20:39:22.2273228Z 2243 | |         "[0x12, 0x34, 0x56, 0x78]", "", "" }
2019-09-28T20:39:22.2278622Z 
2019-09-28T20:39:22.2868130Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-28T20:39:22.2868522Z     --> src/libcore/num/mod.rs:1115:17
2019-09-28T20:39:22.2868737Z      |
2019-09-28T20:39:22.2868737Z      |
2019-09-28T20:39:22.2869037Z 238  | / macro_rules! int_impl {
2019-09-28T20:39:22.2869379Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-28T20:39:22.2870227Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-28T20:39:22.2870597Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-28T20:39:22.2871209Z 1115 | |                 intrinsics::wrapping_add(self, rhs)
2019-09-28T20:39:22.2871542Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-28T20:39:22.2871793Z ...    |
2019-09-28T20:39:22.2872101Z 2224 | |     }
2019-09-28T20:39:22.2872101Z 2224 | |     }
2019-09-28T20:39:22.2872391Z 2225 | | }
2019-09-28T20:39:22.2873166Z      | |_- in this expansion of `int_impl!`
2019-09-28T20:39:22.2873396Z ...
2019-09-28T20:39:22.2873768Z 2248 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
2019-09-28T20:39:22.2874159Z 2249 | |          "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
2019-09-28T20:39:22.2874533Z 2250 | |          "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2019-09-28T20:39:22.2874885Z 2251 | |          "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
2019-09-28T20:39:22.2880500Z 
2019-09-28T20:39:22.3469845Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-28T20:39:22.3470321Z     --> src/libcore/num/mod.rs:1115:17
2019-09-28T20:39:22.3470553Z      |
2019-09-28T20:39:22.3470553Z      |
2019-09-28T20:39:22.3470883Z 238  | / macro_rules! int_impl {
2019-09-28T20:39:22.3471263Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-28T20:39:22.3471659Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-28T20:39:22.3472021Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-28T20:39:22.3472611Z 1115 | |                 intrinsics::wrapping_add(self, rhs)
2019-09-28T20:39:22.3472959Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-28T20:39:22.3473351Z ...    |
2019-09-28T20:39:22.3473633Z 2224 | |     }
2019-09-28T20:39:22.3473633Z 2224 | |     }
2019-09-28T20:39:22.3473915Z 2225 | | }
2019-09-28T20:39:22.3474199Z      | |_- in this expansion of `int_impl!`
2019-09-28T20:39:22.3474414Z ...
2019-09-28T20:39:22.3474763Z 2256 | /     int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
2019-09-28T20:39:22.3475096Z 2257 | |         170141183460469231731687303715884105727, "", "", 16,
2019-09-28T20:39:22.3475485Z 2258 | |         "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
2019-09-28T20:39:22.3475997Z 2259 | |         "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
2019-09-28T20:39:22.3476251Z ...    |
2019-09-28T20:39:22.3476581Z 2262 | |         "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
2019-09-28T20:39:22.3476917Z 2263 | |           0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }
2019-09-28T20:39:22.3482797Z 
2019-09-28T20:39:22.4030703Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-28T20:39:22.4031142Z     --> src/libcore/num/mod.rs:1115:17
2019-09-28T20:39:22.4031375Z      |
2019-09-28T20:39:22.4031375Z      |
2019-09-28T20:39:22.4031698Z 238  | / macro_rules! int_impl {
2019-09-28T20:39:22.4032279Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-28T20:39:22.4032719Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-28T20:39:22.4033084Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-28T20:39:22.4033673Z 1115 | |                 intrinsics::wrapping_add(self, rhs)
2019-09-28T20:39:22.4034028Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-28T20:39:22.4034275Z ...    |
2019-09-28T20:39:22.4034577Z 2224 | |     }
2019-09-28T20:39:22.4034577Z 2224 | |     }
2019-09-28T20:39:22.4034869Z 2225 | | }
2019-09-28T20:39:22.4035174Z      | |_- in this expansion of `int_impl!`
2019-09-28T20:39:22.4035401Z ...
2019-09-28T20:39:22.4035781Z 2286 | /     int_impl! { isize, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "",
2019-09-28T20:39:22.4036168Z 2287 | |         12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
2019-09-28T20:39:22.4036579Z 2288 | |          "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2019-09-28T20:39:22.4036935Z 2289 | |          "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
2019-09-28T20:39:22.4037305Z 2290 | |          usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
2019-09-28T20:39:22.4042404Z 
2019-09-28T20:39:22.4615727Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-28T20:39:22.4616107Z     --> src/libcore/num/mod.rs:1138:17
2019-09-28T20:39:22.4616337Z      |
2019-09-28T20:39:22.4616337Z      |
2019-09-28T20:39:22.4616651Z 238  | / macro_rules! int_impl {
2019-09-28T20:39:22.4617019Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-28T20:39:22.4617410Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-28T20:39:22.4617763Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-28T20:39:22.4618383Z ...    |
2019-09-28T20:39:22.4618791Z 1138 | |                 intrinsics::wrapping_sub(self, rhs)
2019-09-28T20:39:22.4619387Z ...    |
2019-09-28T20:39:22.4619675Z 2224 | |     }
2019-09-28T20:39:22.4620305Z 2225 | | }
2019-09-28T20:39:22.4620305Z 2225 | | }
2019-09-28T20:39:22.4620640Z      | |_- in this expansion of `int_impl!`
2019-09-28T20:39:22.4620855Z ...
2019-09-28T20:39:22.4621237Z 2229 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-09-28T20:39:22.4621586Z 2230 | |         "[0x12]", "[0x12]", "", "" }
2019-09-28T20:39:22.4621961Z 
2019-09-28T20:39:22.5225004Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-28T20:39:22.5225407Z     --> src/libcore/num/mod.rs:1138:17
2019-09-28T20:39:22.5225808Z      |
2019-09-28T20:39:22.5225808Z      |
2019-09-28T20:39:22.5226170Z 238  | / macro_rules! int_impl {
2019-09-28T20:39:22.5226563Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-28T20:39:22.5226965Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-28T20:39:22.5227456Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-28T20:39:22.5228140Z ...    |
2019-09-28T20:39:22.5228494Z 1138 | |                 intrinsics::wrapping_sub(self, rhs)
2019-09-28T20:39:22.5229079Z ...    |
2019-09-28T20:39:22.5229389Z 2224 | |     }
2019-09-28T20:39:22.5230284Z 2225 | | }
2019-09-28T20:39:22.5230284Z 2225 | | }
2019-09-28T20:39:22.5230619Z      | |_- in this expansion of `int_impl!`
2019-09-28T20:39:22.5230834Z ...
2019-09-28T20:39:22.5231235Z 2235 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
2019-09-28T20:39:22.5231610Z 2236 | |         "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
2019-09-28T20:39:22.5288313Z 
2019-09-28T20:39:22.5439658Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-28T20:39:22.5440084Z     --> src/libcore/num/mod.rs:1138:17
2019-09-28T20:39:22.5440315Z      |
2019-09-28T20:39:22.5440315Z      |
2019-09-28T20:39:22.5440648Z 238  | / macro_rules! int_impl {
2019-09-28T20:39:22.5441048Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-28T20:39:22.5441458Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-28T20:39:22.5441805Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-28T20:39:22.5442056Z ...    |
2019-09-28T20:39:22.5442404Z 1138 | |                 intrinsics::wrapping_sub(self, rhs)
2019-09-28T20:39:22.5443137Z ...    |
2019-09-28T20:39:22.5443433Z 2224 | |     }
2019-09-28T20:39:22.5443719Z 2225 | | }
2019-09-28T20:39:22.5443719Z 2225 | | }
2019-09-28T20:39:22.5444033Z      | |_- in this expansion of `int_impl!`
2019-09-28T20:39:22.5444238Z ...
2019-09-28T20:39:22.5444602Z 2241 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
2019-09-28T20:39:22.5444982Z 2242 | |         "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
2019-09-28T20:39:22.5445873Z 2243 | |         "[0x12, 0x34, 0x56, 0x78]", "", "" }
2019-09-28T20:39:22.5450627Z 
2019-09-28T20:39:22.5916401Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-28T20:39:22.5917268Z     --> src/libcore/num/mod.rs:1138:17
2019-09-28T20:39:22.5917967Z      |
2019-09-28T20:39:22.5917967Z      |
2019-09-28T20:39:22.5918583Z 238  | / macro_rules! int_impl {
2019-09-28T20:39:22.5919309Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-28T20:39:22.5920681Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-28T20:39:22.5921444Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-28T20:39:22.5922007Z ...    |
2019-09-28T20:39:22.5922648Z 1138 | |                 intrinsics::wrapping_sub(self, rhs)
2019-09-28T20:39:22.5923996Z ...    |
2019-09-28T20:39:22.5924591Z 2224 | |     }
2019-09-28T20:39:22.5925135Z 2225 | | }
2019-09-28T20:39:22.5925135Z 2225 | | }
2019-09-28T20:39:22.5925729Z      | |_- in this expansion of `int_impl!`
2019-09-28T20:39:22.5926412Z ...
2019-09-28T20:39:22.5927300Z 2248 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
2019-09-28T20:39:22.5928055Z 2249 | |          "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
2019-09-28T20:39:22.5928755Z 2250 | |          "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2019-09-28T20:39:22.5929651Z 2251 | |          "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
2019-09-28T20:39:22.5969058Z 
2019-09-28T20:39:22.6308148Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-28T20:39:22.6309577Z     --> src/libcore/num/mod.rs:1138:17
2019-09-28T20:39:22.6310512Z      |
2019-09-28T20:39:22.6310512Z      |
2019-09-28T20:39:22.6311344Z 238  | / macro_rules! int_impl {
2019-09-28T20:39:22.6311927Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-28T20:39:22.6312529Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-28T20:39:22.6313204Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-28T20:39:22.6313621Z ...    |
2019-09-28T20:39:22.6314119Z 1138 | |                 intrinsics::wrapping_sub(self, rhs)
2019-09-28T20:39:22.6315014Z ...    |
2019-09-28T20:39:22.6315437Z 2224 | |     }
2019-09-28T20:39:22.6315998Z 2225 | | }
2019-09-28T20:39:22.6315998Z 2225 | | }
2019-09-28T20:39:22.6316511Z      | |_- in this expansion of `int_impl!`
2019-09-28T20:39:22.6316894Z ...
2019-09-28T20:39:22.6317398Z 2256 | /     int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
2019-09-28T20:39:22.6318072Z 2257 | |         170141183460469231731687303715884105727, "", "", 16,
2019-09-28T20:39:22.6318664Z 2258 | |         "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
2019-09-28T20:39:22.6320471Z 2259 | |         "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
2019-09-28T20:39:22.6321033Z ...    |
2019-09-28T20:39:22.6321573Z 2262 | |         "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
2019-09-28T20:39:22.6322416Z 2263 | |           0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }
2019-09-28T20:39:22.6346656Z 
2019-09-28T20:39:22.6426682Z    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
2019-09-28T20:39:22.6905529Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-28T20:39:22.6906327Z     --> src/libcore/num/mod.rs:1138:17
2019-09-28T20:39:22.6906327Z     --> src/libcore/num/mod.rs:1138:17
2019-09-28T20:39:22.6906730Z      |
2019-09-28T20:39:22.6907177Z 238  | / macro_rules! int_impl {
2019-09-28T20:39:22.6907724Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-28T20:39:22.6908549Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-28T20:39:22.6909067Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-28T20:39:22.6909879Z ...    |
2019-09-28T20:39:22.6910629Z 1138 | |                 intrinsics::wrapping_sub(self, rhs)
2019-09-28T20:39:22.6911628Z ...    |
2019-09-28T20:39:22.6912147Z 2224 | |     }
2019-09-28T20:39:22.6912644Z 2225 | | }
2019-09-28T20:39:22.6912644Z 2225 | | }
2019-09-28T20:39:22.6913467Z      | |_- in this expansion of `int_impl!`
2019-09-28T20:39:22.6913972Z ...
2019-09-28T20:39:22.6914562Z 2286 | /     int_impl! { isize, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "",
2019-09-28T20:39:22.6915312Z 2287 | |         12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
2019-09-28T20:39:22.6915850Z 2288 | |          "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2019-09-28T20:39:22.6916403Z 2289 | |          "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
2019-09-28T20:39:22.6917114Z 2290 | |          usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
2019-09-28T20:39:22.6923395Z 
2019-09-28T20:39:22.7303912Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-28T20:39:22.7304256Z     --> src/libcore/num/mod.rs:1160:17
2019-09-28T20:39:22.7304471Z      |
2019-09-28T20:39:22.7304471Z      |
2019-09-28T20:39:22.7304814Z 238  | / macro_rules! int_impl {
2019-09-28T20:39:22.7305171Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-28T20:39:22.7305742Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-28T20:39:22.7306081Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-28T20:39:22.7306669Z 1160 | |                 intrinsics::wrapping_mul(self, rhs)
2019-09-28T20:39:22.7307235Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-28T20:39:22.7307479Z ...    |
2019-09-28T20:39:22.7307972Z 2224 | |     }
2019-09-28T20:39:22.7307972Z 2224 | |     }
2019-09-28T20:39:22.7308260Z 2225 | | }
2019-09-28T20:39:22.7308589Z      | |_- in this expansion of `int_impl!`
2019-09-28T20:39:22.7308807Z ...
2019-09-28T20:39:22.7309185Z 2229 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-09-28T20:39:22.7309896Z 2230 | |         "[0x12]", "[0x12]", "", "" }
2019-09-28T20:39:22.7315293Z 
2019-09-28T20:39:22.7708362Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-28T20:39:22.7708929Z     --> src/libcore/num/mod.rs:1160:17
2019-09-28T20:39:22.7709154Z      |
2019-09-28T20:39:22.7709154Z      |
2019-09-28T20:39:22.7709888Z 238  | / macro_rules! int_impl {
2019-09-28T20:39:22.7710333Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-28T20:39:22.7710759Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-28T20:39:22.7711116Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-28T20:39:22.7711728Z 1160 | |                 intrinsics::wrapping_mul(self, rhs)
2019-09-28T20:39:22.7712057Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-28T20:39:22.7712311Z ...    |
2019-09-28T20:39:22.7712631Z 2224 | |     }
2019-09-28T20:39:22.7712631Z 2224 | |     }
2019-09-28T20:39:22.7712918Z 2225 | | }
2019-09-28T20:39:22.7713378Z      | |_- in this expansion of `int_impl!`
2019-09-28T20:39:22.7713615Z ...
2019-09-28T20:39:22.7713992Z 2235 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
2019-09-28T20:39:22.7714350Z 2236 | |         "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
2019-09-28T20:39:22.7721034Z 
2019-09-28T20:39:22.8146401Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-28T20:39:22.8146973Z     --> src/libcore/num/mod.rs:1160:17
2019-09-28T20:39:22.8147205Z      |
2019-09-28T20:39:22.8147205Z      |
2019-09-28T20:39:22.8147577Z 238  | / macro_rules! int_impl {
2019-09-28T20:39:22.8148321Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-28T20:39:22.8148779Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-28T20:39:22.8149147Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-28T20:39:22.8151422Z 1160 | |                 intrinsics::wrapping_mul(self, rhs)
2019-09-28T20:39:22.8152600Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-28T20:39:22.8153039Z ...    |
2019-09-28T20:39:22.8153365Z 2224 | |     }
2019-09-28T20:39:22.8153365Z 2224 | |     }
2019-09-28T20:39:22.8154803Z 2225 | | }
2019-09-28T20:39:22.8155212Z      | |_- in this expansion of `int_impl!`
2019-09-28T20:39:22.8156462Z ...
2019-09-28T20:39:22.8157295Z 2241 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
2019-09-28T20:39:22.8157792Z 2242 | |         "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
2019-09-28T20:39:22.8158141Z 2243 | |         "[0x12, 0x34, 0x56, 0x78]", "", "" }
2019-09-28T20:39:22.8163845Z 
2019-09-28T20:39:22.8746064Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-28T20:39:22.8746490Z     --> src/libcore/num/mod.rs:1160:17
2019-09-28T20:39:22.8746756Z      |
2019-09-28T20:39:22.8746756Z      |
2019-09-28T20:39:22.8747077Z 238  | / macro_rules! int_impl {
2019-09-28T20:39:22.8747465Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-28T20:39:22.8747937Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-28T20:39:22.8748287Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-28T20:39:22.8749144Z 1160 | |                 intrinsics::wrapping_mul(self, rhs)
2019-09-28T20:39:22.8749751Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-28T20:39:22.8750069Z ...    |
2019-09-28T20:39:22.8750359Z 2224 | |     }
2019-09-28T20:39:22.8750359Z 2224 | |     }
2019-09-28T20:39:22.8750645Z 2225 | | }
2019-09-28T20:39:22.8750973Z      | |_- in this expansion of `int_impl!`
2019-09-28T20:39:22.8751199Z ...
2019-09-28T20:39:22.8751595Z 2248 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
2019-09-28T20:39:22.8752113Z 2249 | |          "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
2019-09-28T20:39:22.8752547Z 2250 | |          "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2019-09-28T20:39:22.8752944Z 2251 | |          "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
2019-09-28T20:39:22.8758000Z 
2019-09-28T20:39:22.9432513Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-28T20:39:22.9432908Z     --> src/libcore/num/mod.rs:1160:17
2019-09-28T20:39:22.9433202Z      |
2019-09-28T20:39:22.9433202Z      |
2019-09-28T20:39:22.9433518Z 238  | / macro_rules! int_impl {
2019-09-28T20:39:22.9433907Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-28T20:39:22.9434312Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-28T20:39:22.9434673Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-28T20:39:22.9435290Z 1160 | |                 intrinsics::wrapping_mul(self, rhs)
2019-09-28T20:39:22.9435622Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-28T20:39:22.9435893Z ...    |
2019-09-28T20:39:22.9436185Z 2224 | |     }
2019-09-28T20:39:22.9436185Z 2224 | |     }
2019-09-28T20:39:22.9436472Z 2225 | | }
2019-09-28T20:39:22.9437123Z      | |_- in this expansion of `int_impl!`
2019-09-28T20:39:22.9437344Z ...
2019-09-28T20:39:22.9437717Z 2256 | /     int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
2019-09-28T20:39:22.9438108Z 2257 | |         170141183460469231731687303715884105727, "", "", 16,
2019-09-28T20:39:22.9438514Z 2258 | |         "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
2019-09-28T20:39:22.9438921Z 2259 | |         "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
2019-09-28T20:39:22.9439188Z ...    |
2019-09-28T20:39:22.9439821Z 2262 | |         "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
2019-09-28T20:39:22.9440377Z 2263 | |           0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }
2019-09-28T20:39:22.9451736Z 
2019-09-28T20:39:22.9987761Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-28T20:39:22.9988169Z     --> src/libcore/num/mod.rs:1160:17
2019-09-28T20:39:22.9988473Z      |
2019-09-28T20:39:22.9988473Z      |
2019-09-28T20:39:22.9988829Z 238  | / macro_rules! int_impl {
2019-09-28T20:39:22.9989217Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-28T20:39:22.9989758Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-28T20:39:22.9990108Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-28T20:39:22.9990721Z 1160 | |                 intrinsics::wrapping_mul(self, rhs)
2019-09-28T20:39:22.9991060Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-28T20:39:22.9991330Z ...    |
2019-09-28T20:39:22.9991630Z 2224 | |     }
2019-09-28T20:39:22.9991630Z 2224 | |     }
2019-09-28T20:39:22.9991917Z 2225 | | }
2019-09-28T20:39:22.9992246Z      | |_- in this expansion of `int_impl!`
2019-09-28T20:39:22.9992462Z ...
2019-09-28T20:39:22.9992989Z 2286 | /     int_impl! { isize, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "",
2019-09-28T20:39:22.9993691Z 2287 | |         12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
2019-09-28T20:39:22.9994070Z 2288 | |          "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2019-09-28T20:39:22.9994629Z 2289 | |          "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
2019-09-28T20:39:22.9995159Z 2290 | |          usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
2019-09-28T20:39:23.0001140Z 
2019-09-28T20:39:23.1375754Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-28T20:39:23.1376467Z     --> src/libcore/num/mod.rs:3048:17
2019-09-28T20:39:23.1376889Z      |
2019-09-28T20:39:23.1376889Z      |
2019-09-28T20:39:23.1377555Z 2294 | / macro_rules! uint_impl {
2019-09-28T20:39:23.1378560Z 2295 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2019-09-28T20:39:23.1378925Z 2296 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-28T20:39:23.1379472Z 2297 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-28T20:39:23.1380081Z 3048 | |                 intrinsics::wrapping_add(self, rhs)
2019-09-28T20:39:23.1380438Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-28T20:39:23.1380926Z ...    |
2019-09-28T20:39:23.1381260Z 4028 | |     }
2019-09-28T20:39:23.1381260Z 4028 | |     }
2019-09-28T20:39:23.1381574Z 4029 | | }
2019-09-28T20:39:23.1381889Z      | |_- in this expansion of `uint_impl!`
2019-09-28T20:39:23.1382117Z ...
2019-09-28T20:39:23.1382521Z 4033 | /     uint_impl! { u8, u8, 8, 255, "", "", 2, "0x82", "0xa", "0x12", "0x12", "0x48", "[0x12]",
2019-09-28T20:39:23.1382847Z 4034 | |         "[0x12]", "", "" }
2019-09-28T20:39:23.1411622Z 
2019-09-28T20:39:23.1984959Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-28T20:39:23.1985373Z     --> src/libcore/num/mod.rs:3048:17
2019-09-28T20:39:23.1985605Z      |
2019-09-28T20:39:23.1985605Z      |
2019-09-28T20:39:23.1986190Z 2294 | / macro_rules! uint_impl {
2019-09-28T20:39:23.1986578Z 2295 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2019-09-28T20:39:23.1986953Z 2296 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-28T20:39:23.1987290Z 2297 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-28T20:39:23.1987886Z 3048 | |                 intrinsics::wrapping_add(self, rhs)
2019-09-28T20:39:23.1988232Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-28T20:39:23.1988477Z ...    |
2019-09-28T20:39:23.1988758Z 4028 | |     }
2019-09-28T20:39:23.1988758Z 4028 | |     }
2019-09-28T20:39:23.1989164Z 4029 | | }
2019-09-28T20:39:23.1989513Z      | |_- in this expansion of `uint_impl!`
2019-09-28T20:39:23.1989726Z ...
2019-09-28T20:39:23.1990671Z 4552 | /     uint_impl! { u16, u16, 16, 65535, "", "", 4, "0xa003", "0x3a", "0x1234", "0x3412", "0x2c48",
2019-09-28T20:39:23.1991029Z 4553 | |         "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
2019-09-28T20:39:23.1996052Z 
2019-09-28T20:39:23.2219662Z    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
2019-09-28T20:39:23.2464726Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-28T20:39:23.2465091Z     --> src/libcore/num/mod.rs:3048:17
2019-09-28T20:39:23.2465091Z     --> src/libcore/num/mod.rs:3048:17
2019-09-28T20:39:23.2465319Z      |
2019-09-28T20:39:23.2465635Z 2294 | / macro_rules! uint_impl {
2019-09-28T20:39:23.2465989Z 2295 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2019-09-28T20:39:23.2466369Z 2296 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-28T20:39:23.2466734Z 2297 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-28T20:39:23.2467307Z 3048 | |                 intrinsics::wrapping_add(self, rhs)
2019-09-28T20:39:23.2467620Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-28T20:39:23.2467855Z ...    |
2019-09-28T20:39:23.2468501Z 4028 | |     }
2019-09-28T20:39:23.2468501Z 4028 | |     }
2019-09-28T20:39:23.2468775Z 4029 | | }
2019-09-28T20:39:23.2469080Z      | |_- in this expansion of `uint_impl!`
2019-09-28T20:39:23.2469306Z ...
2019-09-28T20:39:23.2469663Z 4558 | /     uint_impl! { u32, u32, 32, 4294967295, "", "", 8, "0x10000b3", "0xb301", "0x12345678",
2019-09-28T20:39:23.2470484Z 4559 | |         "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]", "[0x12, 0x34, 0x56, 0x78]", "", "" }
2019-09-28T20:39:23.2475634Z 
2019-09-28T20:39:23.2985924Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-28T20:39:23.2986494Z     --> src/libcore/num/mod.rs:3048:17
2019-09-28T20:39:23.2986761Z      |
2019-09-28T20:39:23.2986761Z      |
2019-09-28T20:39:23.2987058Z 2294 | / macro_rules! uint_impl {
2019-09-28T20:39:23.2987384Z 2295 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2019-09-28T20:39:23.2987724Z 2296 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-28T20:39:23.2988045Z 2297 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-28T20:39:23.2988581Z 3048 | |                 intrinsics::wrapping_add(self, rhs)
2019-09-28T20:39:23.2988870Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-28T20:39:23.2989088Z ...    |
2019-09-28T20:39:23.2989358Z 4028 | |     }
2019-09-28T20:39:23.2989358Z 4028 | |     }
2019-09-28T20:39:23.2989605Z 4029 | | }
2019-09-28T20:39:23.2990480Z      | |_- in this expansion of `uint_impl!`
2019-09-28T20:39:23.2990735Z ...
2019-09-28T20:39:23.2991135Z 4564 | /     uint_impl! { u64, u64, 64, 18446744073709551615, "", "", 12, "0xaa00000000006e1", "0x6e10aa",
2019-09-28T20:39:23.2991536Z 4565 | |         "0x1234567890123456", "0x5634129078563412", "0x6a2c48091e6a2c48",
2019-09-28T20:39:23.2991896Z 4566 | |         "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2019-09-28T20:39:23.2992252Z 4567 | |         "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
2019-09-28T20:39:23.2992772Z 4568 | |         "", ""}
2019-09-28T20:39:23.2997817Z 
2019-09-28T20:39:23.3510673Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-28T20:39:23.3511070Z     --> src/libcore/num/mod.rs:3048:17
2019-09-28T20:39:23.3511336Z      |
2019-09-28T20:39:23.3511336Z      |
2019-09-28T20:39:23.3511652Z 2294 | / macro_rules! uint_impl {
2019-09-28T20:39:23.3512026Z 2295 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2019-09-28T20:39:23.3512438Z 2296 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-28T20:39:23.3513006Z 2297 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-28T20:39:23.3513982Z 3048 | |                 intrinsics::wrapping_add(self, rhs)
2019-09-28T20:39:23.3514291Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-28T20:39:23.3514544Z ...    |
2019-09-28T20:39:23.3514829Z 4028 | |     }
2019-09-28T20:39:23.3514829Z 4028 | |     }
2019-09-28T20:39:23.3515098Z 4029 | | }
2019-09-28T20:39:23.3515411Z      | |_- in this expansion of `uint_impl!`
2019-09-28T20:39:23.3515623Z ...
2019-09-28T20:39:23.3516162Z 4573 | /     uint_impl! { u128, u128, 128, 340282366920938463463374607431768211455, "", "", 16,
2019-09-28T20:39:23.3516570Z 4574 | |         "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
2019-09-28T20:39:23.3517128Z 4575 | |         "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
2019-09-28T20:39:23.3517657Z 4576 | |         "[0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, \
2019-09-28T20:39:23.3517923Z ...    |
2019-09-28T20:39:23.3518635Z 4579 | |           0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]",
2019-09-28T20:39:23.3519283Z 4580 | |          "", ""}
2019-09-28T20:39:23.3524461Z 
2019-09-28T20:39:23.3807629Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-28T20:39:23.3808218Z     --> src/libcore/num/mod.rs:3048:17
2019-09-28T20:39:23.3808595Z      |
2019-09-28T20:39:23.3808595Z      |
2019-09-28T20:39:23.3809021Z 2294 | / macro_rules! uint_impl {
2019-09-28T20:39:23.3809369Z 2295 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2019-09-28T20:39:23.3809687Z 2296 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-28T20:39:23.3810490Z 2297 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-28T20:39:23.3811104Z 3048 | |                 intrinsics::wrapping_add(self, rhs)
2019-09-28T20:39:23.3811457Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-28T20:39:23.3811845Z ...    |
2019-09-28T20:39:23.3812203Z 4028 | |     }
2019-09-28T20:39:23.3812203Z 4028 | |     }
2019-09-28T20:39:23.3812494Z 4029 | | }
2019-09-28T20:39:23.3812807Z      | |_- in this expansion of `uint_impl!`
2019-09-28T20:39:23.3813042Z ...
2019-09-28T20:39:23.3813595Z 4601 | /     uint_impl! { usize, u64, 64, 18446744073709551615, "", "", 12, "0xaa00000000006e1", "0x6e10aa",
2019-09-28T20:39:23.3813928Z 4602 | |         "0x1234567890123456", "0x5634129078563412", "0x6a2c48091e6a2c48",
2019-09-28T20:39:23.3814469Z 4603 | |         "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2019-09-28T20:39:23.3815162Z 4604 | |          "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
---
2019-09-28T20:39:28.9878691Z == clock drift check ==
2019-09-28T20:39:28.9893497Z   local time: Sat Sep 28 20:39:28 UTC 2019
2019-09-28T20:39:29.1385293Z   network time: Sat, 28 Sep 2019 20:39:29 GMT
2019-09-28T20:39:29.1393669Z == end clock drift check ==
2019-09-28T20:39:30.8250207Z ##[error]Bash exited with code '1'.
2019-09-28T20:39:30.8411942Z ##[section]Starting: Checkout
2019-09-28T20:39:30.8414721Z ==============================================================================
2019-09-28T20:39:30.8414807Z Task         : Get sources
2019-09-28T20:39:30.8414854Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
