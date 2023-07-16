plain
2019-09-28T23:33:43.2086079Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-28T23:33:43.2301099Z ##[command]git config gc.auto 0
2019-09-28T23:33:43.2374695Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-28T23:33:43.2431242Z ##[command]git config --get-all http.proxy
2019-09-28T23:33:43.2568569Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/61835/merge:refs/remotes/pull/61835/merge
---
2019-09-29T00:03:50.8376695Z    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
2019-09-29T00:03:51.0670459Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-29T00:03:51.0670842Z     --> src/libcore/num/mod.rs:1115:17
2019-09-29T00:03:51.0671094Z      |
2019-09-29T00:03:51.0671854Z 238  | / macro_rules! int_impl {
2019-09-29T00:03:51.0672267Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-29T00:03:51.0673150Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-29T00:03:51.0673548Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-29T00:03:51.0674181Z 1115 | |                 intrinsics::wrapping_add(self, rhs)
2019-09-29T00:03:51.0674532Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-29T00:03:51.0674790Z ...    |
2019-09-29T00:03:51.0675233Z 2224 | |     }
2019-09-29T00:03:51.0675233Z 2224 | |     }
2019-09-29T00:03:51.0675523Z 2225 | | }
2019-09-29T00:03:51.0675813Z      | |_- in this expansion of `int_impl!`
2019-09-29T00:03:51.0676011Z ...
2019-09-29T00:03:51.0676372Z 2229 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-09-29T00:03:51.0676677Z 2230 | |         "[0x12]", "[0x12]", "", "" }
2019-09-29T00:03:51.0682329Z 
2019-09-29T00:03:51.1146881Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-29T00:03:51.1147281Z     --> src/libcore/num/mod.rs:1115:17
2019-09-29T00:03:51.1147506Z      |
2019-09-29T00:03:51.1147506Z      |
2019-09-29T00:03:51.1147801Z 238  | / macro_rules! int_impl {
2019-09-29T00:03:51.1148193Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-29T00:03:51.1148561Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-29T00:03:51.1148892Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-29T00:03:51.1149490Z 1115 | |                 intrinsics::wrapping_add(self, rhs)
2019-09-29T00:03:51.1149815Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-29T00:03:51.1150057Z ...    |
2019-09-29T00:03:51.1150334Z 2224 | |     }
2019-09-29T00:03:51.1150334Z 2224 | |     }
2019-09-29T00:03:51.1150624Z 2225 | | }
2019-09-29T00:03:51.1150921Z      | |_- in this expansion of `int_impl!`
2019-09-29T00:03:51.1151843Z ...
2019-09-29T00:03:51.1152420Z 2235 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
2019-09-29T00:03:51.1152835Z 2236 | |         "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
2019-09-29T00:03:51.1153341Z 
2019-09-29T00:03:51.1749312Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-29T00:03:51.1749713Z     --> src/libcore/num/mod.rs:1115:17
2019-09-29T00:03:51.1749980Z      |
2019-09-29T00:03:51.1749980Z      |
2019-09-29T00:03:51.1750308Z 238  | / macro_rules! int_impl {
2019-09-29T00:03:51.1750685Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-29T00:03:51.1751046Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-29T00:03:51.1751405Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-29T00:03:51.1752476Z 1115 | |                 intrinsics::wrapping_add(self, rhs)
2019-09-29T00:03:51.1752836Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-29T00:03:51.1753095Z ...    |
2019-09-29T00:03:51.1753411Z 2224 | |     }
2019-09-29T00:03:51.1753411Z 2224 | |     }
2019-09-29T00:03:51.1753704Z 2225 | | }
2019-09-29T00:03:51.1754019Z      | |_- in this expansion of `int_impl!`
2019-09-29T00:03:51.1754260Z ...
2019-09-29T00:03:51.1754650Z 2241 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
2019-09-29T00:03:51.1755223Z 2242 | |         "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
2019-09-29T00:03:51.1755569Z 2243 | |         "[0x12, 0x34, 0x56, 0x78]", "", "" }
2019-09-29T00:03:51.1760697Z 
2019-09-29T00:03:51.2270144Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-29T00:03:51.2270502Z     --> src/libcore/num/mod.rs:1115:17
2019-09-29T00:03:51.2270755Z      |
2019-09-29T00:03:51.2270755Z      |
2019-09-29T00:03:51.2271726Z 238  | / macro_rules! int_impl {
2019-09-29T00:03:51.2272307Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-29T00:03:51.2272746Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-29T00:03:51.2273119Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-29T00:03:51.2273720Z 1115 | |                 intrinsics::wrapping_add(self, rhs)
2019-09-29T00:03:51.2274091Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-29T00:03:51.2274354Z ...    |
2019-09-29T00:03:51.2274648Z 2224 | |     }
2019-09-29T00:03:51.2274648Z 2224 | |     }
2019-09-29T00:03:51.2275252Z 2225 | | }
2019-09-29T00:03:51.2275525Z      | |_- in this expansion of `int_impl!`
2019-09-29T00:03:51.2275716Z ...
2019-09-29T00:03:51.2276068Z 2248 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
2019-09-29T00:03:51.2276412Z 2249 | |          "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
2019-09-29T00:03:51.2276786Z 2250 | |          "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2019-09-29T00:03:51.2277114Z 2251 | |          "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
2019-09-29T00:03:51.2277501Z 
2019-09-29T00:03:51.2792095Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-29T00:03:51.2792493Z     --> src/libcore/num/mod.rs:1115:17
2019-09-29T00:03:51.2792806Z      |
2019-09-29T00:03:51.2792806Z      |
2019-09-29T00:03:51.2793136Z 238  | / macro_rules! int_impl {
2019-09-29T00:03:51.2793546Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-29T00:03:51.2793936Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-29T00:03:51.2794283Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-29T00:03:51.2795047Z 1115 | |                 intrinsics::wrapping_add(self, rhs)
2019-09-29T00:03:51.2795679Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-29T00:03:51.2795965Z ...    |
2019-09-29T00:03:51.2796229Z 2224 | |     }
2019-09-29T00:03:51.2796229Z 2224 | |     }
2019-09-29T00:03:51.2796505Z 2225 | | }
2019-09-29T00:03:51.2796790Z      | |_- in this expansion of `int_impl!`
2019-09-29T00:03:51.2796986Z ...
2019-09-29T00:03:51.2797337Z 2256 | /     int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
2019-09-29T00:03:51.2797686Z 2257 | |         170141183460469231731687303715884105727, "", "", 16,
2019-09-29T00:03:51.2798054Z 2258 | |         "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
2019-09-29T00:03:51.2798448Z 2259 | |         "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
2019-09-29T00:03:51.2798681Z ...    |
2019-09-29T00:03:51.2799027Z 2262 | |         "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
2019-09-29T00:03:51.2799359Z 2263 | |           0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }
2019-09-29T00:03:51.2799777Z 
2019-09-29T00:03:51.3348645Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-29T00:03:51.3349012Z     --> src/libcore/num/mod.rs:1115:17
2019-09-29T00:03:51.3349257Z      |
2019-09-29T00:03:51.3349257Z      |
2019-09-29T00:03:51.3349536Z 238  | / macro_rules! int_impl {
2019-09-29T00:03:51.3349880Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-29T00:03:51.3350302Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-29T00:03:51.3350620Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-29T00:03:51.3351171Z 1115 | |                 intrinsics::wrapping_add(self, rhs)
2019-09-29T00:03:51.3351929Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-29T00:03:51.3352221Z ...    |
2019-09-29T00:03:51.3352770Z 2224 | |     }
2019-09-29T00:03:51.3352770Z 2224 | |     }
2019-09-29T00:03:51.3353167Z 2225 | | }
2019-09-29T00:03:51.3353540Z      | |_- in this expansion of `int_impl!`
2019-09-29T00:03:51.3353757Z ...
2019-09-29T00:03:51.3354149Z 2286 | /     int_impl! { isize, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "",
2019-09-29T00:03:51.3354545Z 2287 | |         12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
2019-09-29T00:03:51.3355090Z 2288 | |          "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2019-09-29T00:03:51.3355480Z 2289 | |          "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
2019-09-29T00:03:51.3355830Z 2290 | |          usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
2019-09-29T00:03:51.3356600Z 
2019-09-29T00:03:51.3528103Z    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
2019-09-29T00:03:51.3949096Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-29T00:03:51.3949453Z     --> src/libcore/num/mod.rs:1138:17
2019-09-29T00:03:51.3949453Z     --> src/libcore/num/mod.rs:1138:17
2019-09-29T00:03:51.3949670Z      |
2019-09-29T00:03:51.3949986Z 238  | / macro_rules! int_impl {
2019-09-29T00:03:51.3950335Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-29T00:03:51.3950699Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-29T00:03:51.3951017Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-29T00:03:51.3951244Z ...    |
2019-09-29T00:03:51.3951975Z 1138 | |                 intrinsics::wrapping_sub(self, rhs)
2019-09-29T00:03:51.3952630Z ...    |
2019-09-29T00:03:51.3952941Z 2224 | |     }
2019-09-29T00:03:51.3953227Z 2225 | | }
2019-09-29T00:03:51.3953227Z 2225 | | }
2019-09-29T00:03:51.3953552Z      | |_- in this expansion of `int_impl!`
2019-09-29T00:03:51.3953768Z ...
2019-09-29T00:03:51.3954143Z 2229 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-09-29T00:03:51.3954871Z 2230 | |         "[0x12]", "[0x12]", "", "" }
2019-09-29T00:03:51.3973048Z 
2019-09-29T00:03:51.4589723Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-29T00:03:51.4590075Z     --> src/libcore/num/mod.rs:1138:17
2019-09-29T00:03:51.4590297Z      |
2019-09-29T00:03:51.4590297Z      |
2019-09-29T00:03:51.4590611Z 238  | / macro_rules! int_impl {
2019-09-29T00:03:51.4591008Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-29T00:03:51.4591393Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-29T00:03:51.4592169Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-29T00:03:51.4592435Z ...    |
2019-09-29T00:03:51.4592788Z 1138 | |                 intrinsics::wrapping_sub(self, rhs)
2019-09-29T00:03:51.4593388Z ...    |
2019-09-29T00:03:51.4593704Z 2224 | |     }
2019-09-29T00:03:51.4593995Z 2225 | | }
2019-09-29T00:03:51.4593995Z 2225 | | }
2019-09-29T00:03:51.4594306Z      | |_- in this expansion of `int_impl!`
2019-09-29T00:03:51.4594540Z ...
2019-09-29T00:03:51.4594938Z 2235 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
2019-09-29T00:03:51.4595456Z 2236 | |         "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
2019-09-29T00:03:51.4595850Z 
2019-09-29T00:03:51.5149650Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-29T00:03:51.5149994Z     --> src/libcore/num/mod.rs:1138:17
2019-09-29T00:03:51.5150216Z      |
2019-09-29T00:03:51.5150216Z      |
2019-09-29T00:03:51.5150521Z 238  | / macro_rules! int_impl {
2019-09-29T00:03:51.5150870Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-29T00:03:51.5151238Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-29T00:03:51.5152548Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-29T00:03:51.5152860Z ...    |
2019-09-29T00:03:51.5153221Z 1138 | |                 intrinsics::wrapping_sub(self, rhs)
2019-09-29T00:03:51.5153822Z ...    |
2019-09-29T00:03:51.5154113Z 2224 | |     }
2019-09-29T00:03:51.5154400Z 2225 | | }
2019-09-29T00:03:51.5154400Z 2225 | | }
2019-09-29T00:03:51.5154750Z      | |_- in this expansion of `int_impl!`
2019-09-29T00:03:51.5154972Z ...
2019-09-29T00:03:51.5155489Z 2241 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
2019-09-29T00:03:51.5155851Z 2242 | |         "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
2019-09-29T00:03:51.5156155Z 2243 | |         "[0x12, 0x34, 0x56, 0x78]", "", "" }
2019-09-29T00:03:51.5156534Z 
2019-09-29T00:03:51.5799850Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-29T00:03:51.5800235Z     --> src/libcore/num/mod.rs:1138:17
2019-09-29T00:03:51.5800455Z      |
2019-09-29T00:03:51.5800455Z      |
2019-09-29T00:03:51.5800736Z 238  | / macro_rules! int_impl {
2019-09-29T00:03:51.5801102Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-29T00:03:51.5801453Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-29T00:03:51.5802423Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-29T00:03:51.5802728Z ...    |
2019-09-29T00:03:51.5803064Z 1138 | |                 intrinsics::wrapping_sub(self, rhs)
2019-09-29T00:03:51.5803671Z ...    |
2019-09-29T00:03:51.5803961Z 2224 | |     }
2019-09-29T00:03:51.5804265Z 2225 | | }
2019-09-29T00:03:51.5804265Z 2225 | | }
2019-09-29T00:03:51.5804578Z      | |_- in this expansion of `int_impl!`
2019-09-29T00:03:51.5805061Z ...
2019-09-29T00:03:51.5805561Z 2248 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
2019-09-29T00:03:51.5805987Z 2249 | |          "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
2019-09-29T00:03:51.5806375Z 2250 | |          "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2019-09-29T00:03:51.5806768Z 2251 | |          "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
2019-09-29T00:03:51.5807225Z 
2019-09-29T00:03:51.6349946Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-29T00:03:51.6350299Z     --> src/libcore/num/mod.rs:1138:17
2019-09-29T00:03:51.6350542Z      |
2019-09-29T00:03:51.6350542Z      |
2019-09-29T00:03:51.6350822Z 238  | / macro_rules! int_impl {
2019-09-29T00:03:51.6351167Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-29T00:03:51.6351955Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-29T00:03:51.6352438Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-29T00:03:51.6352729Z ...    |
2019-09-29T00:03:51.6353066Z 1138 | |                 intrinsics::wrapping_sub(self, rhs)
2019-09-29T00:03:51.6353667Z ...    |
2019-09-29T00:03:51.6353955Z 2224 | |     }
2019-09-29T00:03:51.6354256Z 2225 | | }
2019-09-29T00:03:51.6354256Z 2225 | | }
2019-09-29T00:03:51.6354597Z      | |_- in this expansion of `int_impl!`
2019-09-29T00:03:51.6354819Z ...
2019-09-29T00:03:51.6355358Z 2256 | /     int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
2019-09-29T00:03:51.6355703Z 2257 | |         170141183460469231731687303715884105727, "", "", 16,
2019-09-29T00:03:51.6356068Z 2258 | |         "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
2019-09-29T00:03:51.6356447Z 2259 | |         "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
2019-09-29T00:03:51.6356918Z ...    |
2019-09-29T00:03:51.6357360Z 2262 | |         "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
2019-09-29T00:03:51.6357722Z 2263 | |           0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }
2019-09-29T00:03:51.6358119Z 
2019-09-29T00:03:51.6758696Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-29T00:03:51.6759083Z     --> src/libcore/num/mod.rs:1138:17
2019-09-29T00:03:51.6759301Z      |
2019-09-29T00:03:51.6759301Z      |
2019-09-29T00:03:51.6759582Z 238  | / macro_rules! int_impl {
2019-09-29T00:03:51.6759942Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-29T00:03:51.6760282Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-29T00:03:51.6760608Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-29T00:03:51.6760834Z ...    |
2019-09-29T00:03:51.6761135Z 1138 | |                 intrinsics::wrapping_sub(self, rhs)
2019-09-29T00:03:51.6762214Z ...    |
2019-09-29T00:03:51.6762506Z 2224 | |     }
2019-09-29T00:03:51.6762820Z 2225 | | }
2019-09-29T00:03:51.6762820Z 2225 | | }
2019-09-29T00:03:51.6763132Z      | |_- in this expansion of `int_impl!`
2019-09-29T00:03:51.6763349Z ...
2019-09-29T00:03:51.6763743Z 2286 | /     int_impl! { isize, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "",
2019-09-29T00:03:51.6764159Z 2287 | |         12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
2019-09-29T00:03:51.6764573Z 2288 | |          "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2019-09-29T00:03:51.6764936Z 2289 | |          "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
2019-09-29T00:03:51.6765414Z 2290 | |          usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
2019-09-29T00:03:51.6765991Z 
2019-09-29T00:03:51.7106897Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-29T00:03:51.7107258Z     --> src/libcore/num/mod.rs:1160:17
2019-09-29T00:03:51.7107467Z      |
2019-09-29T00:03:51.7107467Z      |
2019-09-29T00:03:51.7107761Z 238  | / macro_rules! int_impl {
2019-09-29T00:03:51.7108099Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-29T00:03:51.7108434Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-29T00:03:51.7108782Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-29T00:03:51.7109296Z 1160 | |                 intrinsics::wrapping_mul(self, rhs)
2019-09-29T00:03:51.7109605Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-29T00:03:51.7109836Z ...    |
2019-09-29T00:03:51.7110108Z 2224 | |     }
2019-09-29T00:03:51.7110108Z 2224 | |     }
2019-09-29T00:03:51.7110365Z 2225 | | }
2019-09-29T00:03:51.7110648Z      | |_- in this expansion of `int_impl!`
2019-09-29T00:03:51.7110870Z ...
2019-09-29T00:03:51.7111659Z 2229 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-09-29T00:03:51.7112040Z 2230 | |         "[0x12]", "[0x12]", "", "" }
2019-09-29T00:03:51.7112450Z 
2019-09-29T00:03:51.7477704Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-29T00:03:51.7478083Z     --> src/libcore/num/mod.rs:1160:17
2019-09-29T00:03:51.7478288Z      |
2019-09-29T00:03:51.7478288Z      |
2019-09-29T00:03:51.7478588Z 238  | / macro_rules! int_impl {
2019-09-29T00:03:51.7479104Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-29T00:03:51.7479448Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-29T00:03:51.7479779Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-29T00:03:51.7480913Z 1160 | |                 intrinsics::wrapping_mul(self, rhs)
2019-09-29T00:03:51.7481363Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-29T00:03:51.7481641Z ...    |
2019-09-29T00:03:51.7481938Z 2224 | |     }
2019-09-29T00:03:51.7481938Z 2224 | |     }
2019-09-29T00:03:51.7482214Z 2225 | | }
2019-09-29T00:03:51.7482512Z      | |_- in this expansion of `int_impl!`
2019-09-29T00:03:51.7482740Z ...
2019-09-29T00:03:51.7483126Z 2235 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
2019-09-29T00:03:51.7483488Z 2236 | |         "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
2019-09-29T00:03:51.7483910Z 
2019-09-29T00:03:51.7870021Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-29T00:03:51.7870381Z     --> src/libcore/num/mod.rs:1160:17
2019-09-29T00:03:51.7870593Z      |
2019-09-29T00:03:51.7870593Z      |
2019-09-29T00:03:51.7870871Z 238  | / macro_rules! int_impl {
2019-09-29T00:03:51.7871281Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-29T00:03:51.7872079Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-29T00:03:51.7872501Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-29T00:03:51.7873084Z 1160 | |                 intrinsics::wrapping_mul(self, rhs)
2019-09-29T00:03:51.7873438Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-29T00:03:51.7873705Z ...    |
2019-09-29T00:03:51.7874001Z 2224 | |     }
2019-09-29T00:03:51.7874001Z 2224 | |     }
2019-09-29T00:03:51.7874311Z 2225 | | }
2019-09-29T00:03:51.7874619Z      | |_- in this expansion of `int_impl!`
2019-09-29T00:03:51.7874855Z ...
2019-09-29T00:03:51.7875395Z 2241 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
2019-09-29T00:03:51.7875752Z 2242 | |         "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
2019-09-29T00:03:51.7876323Z 2243 | |         "[0x12, 0x34, 0x56, 0x78]", "", "" }
2019-09-29T00:03:51.7876796Z 
2019-09-29T00:03:51.8215986Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-29T00:03:51.8216312Z     --> src/libcore/num/mod.rs:1160:17
2019-09-29T00:03:51.8216524Z      |
2019-09-29T00:03:51.8216524Z      |
2019-09-29T00:03:51.8216831Z 238  | / macro_rules! int_impl {
2019-09-29T00:03:51.8217179Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-29T00:03:51.8217586Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-29T00:03:51.8217904Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-29T00:03:51.8218460Z 1160 | |                 intrinsics::wrapping_mul(self, rhs)
2019-09-29T00:03:51.8218760Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-29T00:03:51.8218989Z ...    |
2019-09-29T00:03:51.8219283Z 2224 | |     }
2019-09-29T00:03:51.8219283Z 2224 | |     }
2019-09-29T00:03:51.8219548Z 2225 | | }
2019-09-29T00:03:51.8219851Z      | |_- in this expansion of `int_impl!`
2019-09-29T00:03:51.8220051Z ...
2019-09-29T00:03:51.8220395Z 2248 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
2019-09-29T00:03:51.8220767Z 2249 | |          "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
2019-09-29T00:03:51.8221120Z 2250 | |          "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2019-09-29T00:03:51.8221862Z 2251 | |          "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
2019-09-29T00:03:51.8222376Z 
2019-09-29T00:03:51.8669605Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-29T00:03:51.8669970Z     --> src/libcore/num/mod.rs:1160:17
2019-09-29T00:03:51.8670184Z      |
2019-09-29T00:03:51.8670184Z      |
2019-09-29T00:03:51.8670484Z 238  | / macro_rules! int_impl {
2019-09-29T00:03:51.8671208Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-29T00:03:51.8671977Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-29T00:03:51.8672421Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-29T00:03:51.8673034Z 1160 | |                 intrinsics::wrapping_mul(self, rhs)
2019-09-29T00:03:51.8673371Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-29T00:03:51.8673640Z ...    |
2019-09-29T00:03:51.8673959Z 2224 | |     }
2019-09-29T00:03:51.8673959Z 2224 | |     }
2019-09-29T00:03:51.8674253Z 2225 | | }
2019-09-29T00:03:51.8674562Z      | |_- in this expansion of `int_impl!`
2019-09-29T00:03:51.8674802Z ...
2019-09-29T00:03:51.8675329Z 2256 | /     int_impl! { i128, i128, u128, 128, -170141183460469231731687303715884105728,
2019-09-29T00:03:51.8675656Z 2257 | |         170141183460469231731687303715884105727, "", "", 16,
2019-09-29T00:03:51.8676063Z 2258 | |         "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
2019-09-29T00:03:51.8676416Z 2259 | |         "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
2019-09-29T00:03:51.8676665Z ...    |
2019-09-29T00:03:51.8676985Z 2262 | |         "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56, \
2019-09-29T00:03:51.8677315Z 2263 | |           0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]", "", "" }
2019-09-29T00:03:51.8677726Z 
2019-09-29T00:03:51.9272595Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-29T00:03:51.9273017Z     --> src/libcore/num/mod.rs:1160:17
2019-09-29T00:03:51.9273253Z      |
2019-09-29T00:03:51.9273253Z      |
2019-09-29T00:03:51.9273580Z 238  | / macro_rules! int_impl {
2019-09-29T00:03:51.9273964Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-09-29T00:03:51.9274346Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-29T00:03:51.9275084Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-29T00:03:51.9275986Z 1160 | |                 intrinsics::wrapping_mul(self, rhs)
2019-09-29T00:03:51.9276306Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-29T00:03:51.9276707Z ...    |
2019-09-29T00:03:51.9276996Z 2224 | |     }
2019-09-29T00:03:51.9276996Z 2224 | |     }
2019-09-29T00:03:51.9277266Z 2225 | | }
2019-09-29T00:03:51.9277567Z      | |_- in this expansion of `int_impl!`
2019-09-29T00:03:51.9277796Z ...
2019-09-29T00:03:51.9278151Z 2286 | /     int_impl! { isize, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "",
2019-09-29T00:03:51.9278517Z 2287 | |         12, "0xaa00000000006e1", "0x6e10aa",  "0x1234567890123456", "0x5634129078563412",
2019-09-29T00:03:51.9278900Z 2288 | |          "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2019-09-29T00:03:51.9279235Z 2289 | |          "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
2019-09-29T00:03:51.9279607Z 2290 | |          usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
2019-09-29T00:03:51.9280625Z 
2019-09-29T00:03:52.0453168Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-29T00:03:52.0453520Z     --> src/libcore/num/mod.rs:3048:17
2019-09-29T00:03:52.0453748Z      |
2019-09-29T00:03:52.0453748Z      |
2019-09-29T00:03:52.0454052Z 2294 | / macro_rules! uint_impl {
2019-09-29T00:03:52.0454429Z 2295 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2019-09-29T00:03:52.0454793Z 2296 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-29T00:03:52.0455116Z 2297 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-29T00:03:52.0455685Z 3048 | |                 intrinsics::wrapping_add(self, rhs)
2019-09-29T00:03:52.0455982Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-29T00:03:52.0456494Z ...    |
2019-09-29T00:03:52.0456870Z 4028 | |     }
2019-09-29T00:03:52.0456870Z 4028 | |     }
2019-09-29T00:03:52.0457170Z 4029 | | }
2019-09-29T00:03:52.0457477Z      | |_- in this expansion of `uint_impl!`
2019-09-29T00:03:52.0457678Z ...
2019-09-29T00:03:52.0458019Z 4033 | /     uint_impl! { u8, u8, 8, 255, "", "", 2, "0x82", "0xa", "0x12", "0x12", "0x48", "[0x12]",
2019-09-29T00:03:52.0458323Z 4034 | |         "[0x12]", "", "" }
2019-09-29T00:03:52.0462741Z 
2019-09-29T00:03:52.0834360Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-29T00:03:52.0834731Z     --> src/libcore/num/mod.rs:3048:17
2019-09-29T00:03:52.0834970Z      |
2019-09-29T00:03:52.0834970Z      |
2019-09-29T00:03:52.0835468Z 2294 | / macro_rules! uint_impl {
2019-09-29T00:03:52.0835805Z 2295 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2019-09-29T00:03:52.0836148Z 2296 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-29T00:03:52.0836471Z 2297 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-29T00:03:52.0837016Z 3048 | |                 intrinsics::wrapping_add(self, rhs)
2019-09-29T00:03:52.0837306Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-29T00:03:52.0837532Z ...    |
2019-09-29T00:03:52.0837806Z 4028 | |     }
2019-09-29T00:03:52.0837806Z 4028 | |     }
2019-09-29T00:03:52.0838055Z 4029 | | }
2019-09-29T00:03:52.0838363Z      | |_- in this expansion of `uint_impl!`
2019-09-29T00:03:52.0838558Z ...
2019-09-29T00:03:52.0838905Z 4552 | /     uint_impl! { u16, u16, 16, 65535, "", "", 4, "0xa003", "0x3a", "0x1234", "0x3412", "0x2c48",
2019-09-29T00:03:52.0839228Z 4553 | |         "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
2019-09-29T00:03:52.0844313Z 
2019-09-29T00:03:52.1197122Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-29T00:03:52.1197541Z     --> src/libcore/num/mod.rs:3048:17
2019-09-29T00:03:52.1198046Z      |
2019-09-29T00:03:52.1198046Z      |
2019-09-29T00:03:52.1198480Z 2294 | / macro_rules! uint_impl {
2019-09-29T00:03:52.1199162Z 2295 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2019-09-29T00:03:52.1199737Z 2296 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-29T00:03:52.1200147Z 2297 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-29T00:03:52.1200702Z 3048 | |                 intrinsics::wrapping_add(self, rhs)
2019-09-29T00:03:52.1201030Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-29T00:03:52.1201260Z ...    |
2019-09-29T00:03:52.1202147Z 4028 | |     }
2019-09-29T00:03:52.1202147Z 4028 | |     }
2019-09-29T00:03:52.1202444Z 4029 | | }
2019-09-29T00:03:52.1202763Z      | |_- in this expansion of `uint_impl!`
2019-09-29T00:03:52.1203005Z ...
2019-09-29T00:03:52.1203390Z 4558 | /     uint_impl! { u32, u32, 32, 4294967295, "", "", 8, "0x10000b3", "0xb301", "0x12345678",
2019-09-29T00:03:52.1203846Z 4559 | |         "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]", "[0x12, 0x34, 0x56, 0x78]", "", "" }
2019-09-29T00:03:52.1204351Z 
2019-09-29T00:03:52.1538460Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-29T00:03:52.1538875Z     --> src/libcore/num/mod.rs:3048:17
2019-09-29T00:03:52.1539097Z      |
2019-09-29T00:03:52.1539097Z      |
2019-09-29T00:03:52.1539417Z 2294 | / macro_rules! uint_impl {
2019-09-29T00:03:52.1539872Z 2295 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2019-09-29T00:03:52.1540487Z 2296 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-29T00:03:52.1540880Z 2297 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-29T00:03:52.1542212Z 3048 | |                 intrinsics::wrapping_add(self, rhs)
2019-09-29T00:03:52.1542548Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-29T00:03:52.1543077Z ...    |
2019-09-29T00:03:52.1543507Z 4028 | |     }
2019-09-29T00:03:52.1543507Z 4028 | |     }
2019-09-29T00:03:52.1543836Z 4029 | | }
2019-09-29T00:03:52.1544170Z      | |_- in this expansion of `uint_impl!`
2019-09-29T00:03:52.1544390Z ...
2019-09-29T00:03:52.1544780Z 4564 | /     uint_impl! { u64, u64, 64, 18446744073709551615, "", "", 12, "0xaa00000000006e1", "0x6e10aa",
2019-09-29T00:03:52.1545434Z 4565 | |         "0x1234567890123456", "0x5634129078563412", "0x6a2c48091e6a2c48",
2019-09-29T00:03:52.1545791Z 4566 | |         "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2019-09-29T00:03:52.1546126Z 4567 | |         "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
2019-09-29T00:03:52.1546430Z 4568 | |         "", ""}
2019-09-29T00:03:52.1546800Z 
2019-09-29T00:03:52.1931837Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-29T00:03:52.1932312Z     --> src/libcore/num/mod.rs:3048:17
2019-09-29T00:03:52.1933381Z      |
2019-09-29T00:03:52.1933381Z      |
2019-09-29T00:03:52.1933773Z 2294 | / macro_rules! uint_impl {
2019-09-29T00:03:52.1934195Z 2295 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2019-09-29T00:03:52.1934624Z 2296 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-29T00:03:52.1935001Z 2297 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-29T00:03:52.1935681Z 3048 | |                 intrinsics::wrapping_add(self, rhs)
2019-09-29T00:03:52.1936070Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-29T00:03:52.1936379Z ...    |
2019-09-29T00:03:52.1936704Z 4028 | |     }
2019-09-29T00:03:52.1936704Z 4028 | |     }
2019-09-29T00:03:52.1937027Z 4029 | | }
2019-09-29T00:03:52.1937402Z      | |_- in this expansion of `uint_impl!`
2019-09-29T00:03:52.1937662Z ...
2019-09-29T00:03:52.1938067Z 4573 | /     uint_impl! { u128, u128, 128, 340282366920938463463374607431768211455, "", "", 16,
2019-09-29T00:03:52.1939005Z 4574 | |         "0x13f40000000000000000000000004f76", "0x4f7613f4", "0x12345678901234567890123456789012",
2019-09-29T00:03:52.1939487Z 4575 | |         "0x12907856341290785634129078563412", "0x48091e6a2c48091e6a2c48091e6a2c48",
2019-09-29T00:03:52.1939912Z 4576 | |         "[0x12, 0x90, 0x78, 0x56, 0x34, 0x12, 0x90, 0x78, \
2019-09-29T00:03:52.1940203Z ...    |
2019-09-29T00:03:52.1940586Z 4579 | |           0x78, 0x90, 0x12, 0x34, 0x56, 0x78, 0x90, 0x12]",
2019-09-29T00:03:52.1940952Z 4580 | |          "", ""}
2019-09-29T00:03:52.1941710Z 
2019-09-29T00:03:52.2317764Z error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
2019-09-29T00:03:52.2318227Z     --> src/libcore/num/mod.rs:3048:17
2019-09-29T00:03:52.2318915Z      |
2019-09-29T00:03:52.2318915Z      |
2019-09-29T00:03:52.2319477Z 2294 | / macro_rules! uint_impl {
2019-09-29T00:03:52.2319911Z 2295 | |     ($SelfT:ty, $ActualT:ty, $BITS:expr, $MaxV:expr, $Feature:expr, $EndFeature:expr,
2019-09-29T00:03:52.2320298Z 2296 | |         $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-09-29T00:03:52.2320712Z 2297 | |         $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-09-29T00:03:52.2321330Z 3048 | |                 intrinsics::wrapping_add(self, rhs)
2019-09-29T00:03:52.2321659Z      | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-29T00:03:52.2321918Z ...    |
2019-09-29T00:03:52.2322951Z 4028 | |     }
2019-09-29T00:03:52.2322951Z 4028 | |     }
2019-09-29T00:03:52.2323294Z 4029 | | }
2019-09-29T00:03:52.2323670Z      | |_- in this expansion of `uint_impl!`
2019-09-29T00:03:52.2323952Z ...
2019-09-29T00:03:52.2324381Z 4601 | /     uint_impl! { usize, u64, 64, 18446744073709551615, "", "", 12, "0xaa00000000006e1", "0x6e10aa",
2019-09-29T00:03:52.2324792Z 4602 | |         "0x1234567890123456", "0x5634129078563412", "0x6a2c48091e6a2c48",
2019-09-29T00:03:52.2325208Z 4603 | |         "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2019-09-29T00:03:52.2325761Z 4604 | |          "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]",
2019-09-29T00:03:52.2326683Z 4605 | |         usize_isize_to_xe_bytes_doc!(), usize_isize_from_xe_bytes_doc!() }
---
2019-09-29T00:03:57.7134523Z == clock drift check ==
2019-09-29T00:03:57.7161356Z   local time: Sun Sep 29 00:03:57 UTC 2019
2019-09-29T00:03:57.8117041Z   network time: Sun, 29 Sep 2019 00:03:57 GMT
2019-09-29T00:03:57.8127858Z == end clock drift check ==
2019-09-29T00:03:59.1900818Z ##[error]Bash exited with code '1'.
2019-09-29T00:03:59.1937712Z ##[section]Starting: Checkout
2019-09-29T00:03:59.1957871Z ==============================================================================
2019-09-29T00:03:59.1957931Z Task         : Get sources
2019-09-29T00:03:59.1957972Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
