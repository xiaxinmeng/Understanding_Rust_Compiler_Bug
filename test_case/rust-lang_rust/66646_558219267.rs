plain
2019-11-25T15:26:45.1093289Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-25T15:26:45.1111214Z ##[command]git config gc.auto 0
2019-11-25T15:26:45.1116866Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-25T15:26:45.1119980Z ##[command]git config --get-all http.proxy
2019-11-25T15:26:46.6375933Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66646/merge:refs/remotes/pull/66646/merge
---
2019-11-25T15:55:45.6371154Z    Compiling autocfg v0.1.6
2019-11-25T15:55:45.6946084Z warning: Constant evaluating a complex constant, this might take some time
2019-11-25T15:55:45.6946495Z     --> src/libcore/num/mod.rs:2066:52
2019-11-25T15:55:45.6947006Z      |
2019-11-25T15:55:45.6947320Z 238  | / macro_rules! int_impl {
2019-11-25T15:55:45.6947886Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-11-25T15:55:45.6948278Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-11-25T15:55:45.6948642Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-11-25T15:55:45.6948878Z ...    |
2019-11-25T15:55:45.6949226Z 2066 | |             pub const fn to_be_bytes(self) -> [u8; mem::size_of::<Self>()] {
2019-11-25T15:55:45.6949822Z ...    |
2019-11-25T15:55:45.6950110Z 2236 | |     }
2019-11-25T15:55:45.6951675Z 2237 | | }
2019-11-25T15:55:45.6951675Z 2237 | | }
2019-11-25T15:55:45.6951996Z      | |_- in this expansion of `int_impl!`
2019-11-25T15:55:45.6952224Z ...
2019-11-25T15:55:45.6952592Z 2241 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-11-25T15:55:45.6952917Z 2242 | |         "[0x12]", "[0x12]", "", "" }
2019-11-25T15:55:45.7008368Z 
2019-11-25T15:55:45.7008935Z error[E0080]: evaluation of constant value failed
2019-11-25T15:55:45.7009269Z    --> src/libcore/mem/mod.rs:275:5
2019-11-25T15:55:45.7009489Z     |
2019-11-25T15:55:45.7009489Z     |
2019-11-25T15:55:45.7009762Z 275 |     intrinsics::size_of::<T>()
2019-11-25T15:55:45.7010488Z     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ duplicate interpreter state observed here, const evaluation will never terminate
2019-11-25T15:55:45.7010543Z 
2019-11-25T15:55:46.8106025Z    Compiling std v0.0.0 (/checkout/src/libstd)
2019-11-25T15:55:46.9706581Z warning: Constant evaluating a complex constant, this might take some time
2019-11-25T15:55:46.9707077Z     --> src/libcore/num/mod.rs:2086:52
2019-11-25T15:55:46.9707330Z      |
2019-11-25T15:55:46.9707974Z 238  | / macro_rules! int_impl {
2019-11-25T15:55:46.9708447Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-11-25T15:55:46.9708846Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-11-25T15:55:46.9712573Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-11-25T15:55:46.9715691Z ...    |
2019-11-25T15:55:46.9716070Z 2086 | |             pub const fn to_le_bytes(self) -> [u8; mem::size_of::<Self>()] {
2019-11-25T15:55:46.9716709Z ...    |
2019-11-25T15:55:46.9716989Z 2236 | |     }
2019-11-25T15:55:46.9717299Z 2237 | | }
2019-11-25T15:55:46.9717299Z 2237 | | }
2019-11-25T15:55:46.9717618Z      | |_- in this expansion of `int_impl!`
2019-11-25T15:55:46.9717838Z ...
2019-11-25T15:55:46.9718225Z 2241 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-11-25T15:55:46.9718638Z 2242 | |         "[0x12]", "[0x12]", "", "" }
2019-11-25T15:55:46.9719037Z 
2019-11-25T15:55:47.2441079Z    Compiling cmake v0.1.38
2019-11-25T15:55:48.2625074Z warning: Constant evaluating a complex constant, this might take some time
2019-11-25T15:55:48.2625525Z     --> src/libcore/num/mod.rs:2121:52
2019-11-25T15:55:48.2625525Z     --> src/libcore/num/mod.rs:2121:52
2019-11-25T15:55:48.2625765Z      |
2019-11-25T15:55:48.2626106Z 238  | / macro_rules! int_impl {
2019-11-25T15:55:48.2626506Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-11-25T15:55:48.2626910Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-11-25T15:55:48.2627284Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-11-25T15:55:48.2627541Z ...    |
2019-11-25T15:55:48.2627922Z 2121 | |             pub const fn to_ne_bytes(self) -> [u8; mem::size_of::<Self>()] {
2019-11-25T15:55:48.2628513Z ...    |
2019-11-25T15:55:48.2629037Z 2236 | |     }
2019-11-25T15:55:48.2629386Z 2237 | | }
2019-11-25T15:55:48.2629386Z 2237 | | }
2019-11-25T15:55:48.2629696Z      | |_- in this expansion of `int_impl!`
2019-11-25T15:55:48.2630085Z ...
2019-11-25T15:55:48.2630597Z 2241 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-11-25T15:55:48.2630935Z 2242 | |         "[0x12]", "[0x12]", "", "" }
2019-11-25T15:55:48.2631305Z 
2019-11-25T15:55:49.3697130Z    Compiling compiler_builtins v0.1.18
2019-11-25T15:55:49.5461494Z warning: Constant evaluating a complex constant, this might take some time
2019-11-25T15:55:49.5461857Z     --> src/libcore/num/mod.rs:2155:52
2019-11-25T15:55:49.5461857Z     --> src/libcore/num/mod.rs:2155:52
2019-11-25T15:55:49.5462073Z      |
2019-11-25T15:55:49.5462406Z 238  | / macro_rules! int_impl {
2019-11-25T15:55:49.5462763Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-11-25T15:55:49.5463121Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-11-25T15:55:49.5463470Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-11-25T15:55:49.5463700Z ...    |
2019-11-25T15:55:49.5464425Z 2155 | |             pub const fn from_be_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
2019-11-25T15:55:49.5465043Z ...    |
2019-11-25T15:55:49.5465325Z 2236 | |     }
2019-11-25T15:55:49.5465601Z 2237 | | }
2019-11-25T15:55:49.5465601Z 2237 | | }
2019-11-25T15:55:49.5465957Z      | |_- in this expansion of `int_impl!`
2019-11-25T15:55:49.5466164Z ...
2019-11-25T15:55:49.5466538Z 2241 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-11-25T15:55:49.5466893Z 2242 | |         "[0x12]", "[0x12]", "", "" }
2019-11-25T15:55:49.5471475Z 
2019-11-25T15:55:50.8380718Z warning: Constant evaluating a complex constant, this might take some time
2019-11-25T15:55:50.8381932Z     --> src/libcore/num/mod.rs:2188:52
2019-11-25T15:55:50.8383141Z      |
2019-11-25T15:55:50.8383141Z      |
2019-11-25T15:55:50.8383865Z 238  | / macro_rules! int_impl {
2019-11-25T15:55:50.8384480Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-11-25T15:55:50.8385268Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-11-25T15:55:50.8386660Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-11-25T15:55:50.8387728Z ...    |
2019-11-25T15:55:50.8388118Z 2188 | |             pub const fn from_le_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
2019-11-25T15:55:50.8388737Z ...    |
2019-11-25T15:55:50.8389030Z 2236 | |     }
2019-11-25T15:55:50.8389329Z 2237 | | }
2019-11-25T15:55:50.8389329Z 2237 | | }
2019-11-25T15:55:50.8389629Z      | |_- in this expansion of `int_impl!`
2019-11-25T15:55:50.8389852Z ...
2019-11-25T15:55:50.8390241Z 2241 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-11-25T15:55:50.8390567Z 2242 | |         "[0x12]", "[0x12]", "", "" }
2019-11-25T15:55:50.8390972Z 
2019-11-25T15:55:51.2757657Z    Compiling backtrace-sys v0.1.32
2019-11-25T15:55:52.1545568Z warning: Constant evaluating a complex constant, this might take some time
2019-11-25T15:55:52.1546051Z     --> src/libcore/num/mod.rs:2231:52
2019-11-25T15:55:52.1546051Z     --> src/libcore/num/mod.rs:2231:52
2019-11-25T15:55:52.1546325Z      |
2019-11-25T15:55:52.1546628Z 238  | / macro_rules! int_impl {
2019-11-25T15:55:52.1547064Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-11-25T15:55:52.1547461Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-11-25T15:55:52.1547844Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-11-25T15:55:52.1548096Z ...    |
2019-11-25T15:55:52.1548458Z 2231 | |             pub const fn from_ne_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
2019-11-25T15:55:52.1549317Z ...    |
2019-11-25T15:55:52.1549602Z 2236 | |     }
2019-11-25T15:55:52.1549910Z 2237 | | }
2019-11-25T15:55:52.1549910Z 2237 | | }
2019-11-25T15:55:52.1550559Z      | |_- in this expansion of `int_impl!`
2019-11-25T15:55:52.1550952Z ...
2019-11-25T15:55:52.1551324Z 2241 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-11-25T15:55:52.1551628Z 2242 | |         "[0x12]", "[0x12]", "", "" }
2019-11-25T15:55:52.1552012Z 
2019-11-25T15:55:52.1940437Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2019-11-25T15:55:53.1243434Z    Compiling hashbrown v0.6.2
2019-11-25T15:55:53.3704890Z    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
2019-11-25T15:55:53.3704890Z    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
2019-11-25T15:55:53.4395997Z warning: Constant evaluating a complex constant, this might take some time
2019-11-25T15:55:53.4396530Z     --> src/libcore/num/mod.rs:2066:52
2019-11-25T15:55:53.4396802Z      |
2019-11-25T15:55:53.4397112Z 238  | / macro_rules! int_impl {
2019-11-25T15:55:53.4397524Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-11-25T15:55:53.4397949Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-11-25T15:55:53.4398303Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-11-25T15:55:53.4398573Z ...    |
2019-11-25T15:55:53.4398925Z 2066 | |             pub const fn to_be_bytes(self) -> [u8; mem::size_of::<Self>()] {
2019-11-25T15:55:53.4399547Z ...    |
2019-11-25T15:55:53.4399830Z 2236 | |     }
2019-11-25T15:55:53.4400130Z 2237 | | }
2019-11-25T15:55:53.4400130Z 2237 | | }
2019-11-25T15:55:53.4400449Z      | |_- in this expansion of `int_impl!`
2019-11-25T15:55:53.4400655Z ...
2019-11-25T15:55:53.4401073Z 2247 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
2019-11-25T15:55:53.4401431Z 2248 | |         "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
2019-11-25T15:55:53.4402288Z 
2019-11-25T15:55:53.8837487Z    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
2019-11-25T15:55:54.3726876Z    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
2019-11-25T15:55:54.7141305Z warning: Constant evaluating a complex constant, this might take some time
2019-11-25T15:55:54.7141305Z warning: Constant evaluating a complex constant, this might take some time
2019-11-25T15:55:54.7142099Z     --> src/libcore/num/mod.rs:2086:52
2019-11-25T15:55:54.7143152Z      |
2019-11-25T15:55:54.7144131Z 238  | / macro_rules! int_impl {
2019-11-25T15:55:54.7144969Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-11-25T15:55:54.7145382Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-11-25T15:55:54.7145730Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-11-25T15:55:54.7146012Z ...    |
2019-11-25T15:55:54.7146368Z 2086 | |             pub const fn to_le_bytes(self) -> [u8; mem::size_of::<Self>()] {
2019-11-25T15:55:54.7146986Z ...    |
2019-11-25T15:55:54.7147266Z 2236 | |     }
2019-11-25T15:55:54.7147543Z 2237 | | }
2019-11-25T15:55:54.7147543Z 2237 | | }
2019-11-25T15:55:54.7148087Z      | |_- in this expansion of `int_impl!`
2019-11-25T15:55:54.7148685Z ...
2019-11-25T15:55:54.7149215Z 2247 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
2019-11-25T15:55:54.7149590Z 2248 | |         "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
2019-11-25T15:55:54.7150000Z 
2019-11-25T15:55:54.9072574Z    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
2019-11-25T15:55:55.9592983Z warning: Constant evaluating a complex constant, this might take some time
2019-11-25T15:55:55.9593535Z     --> src/libcore/num/mod.rs:2121:52
2019-11-25T15:55:55.9593535Z     --> src/libcore/num/mod.rs:2121:52
2019-11-25T15:55:55.9593759Z      |
2019-11-25T15:55:55.9594078Z 238  | / macro_rules! int_impl {
2019-11-25T15:55:55.9594451Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-11-25T15:55:55.9595233Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-11-25T15:55:55.9595642Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-11-25T15:55:55.9595891Z ...    |
2019-11-25T15:55:55.9596500Z 2121 | |             pub const fn to_ne_bytes(self) -> [u8; mem::size_of::<Self>()] {
2019-11-25T15:55:55.9597511Z ...    |
2019-11-25T15:55:55.9597819Z 2236 | |     }
2019-11-25T15:55:55.9598110Z 2237 | | }
2019-11-25T15:55:55.9598110Z 2237 | | }
2019-11-25T15:55:55.9598437Z      | |_- in this expansion of `int_impl!`
2019-11-25T15:55:55.9598644Z ...
2019-11-25T15:55:55.9599041Z 2247 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
2019-11-25T15:55:55.9599583Z 2248 | |         "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
2019-11-25T15:55:55.9599976Z 
2019-11-25T15:55:57.2083856Z warning: Constant evaluating a complex constant, this might take some time
2019-11-25T15:55:57.2084266Z     --> src/libcore/num/mod.rs:2155:52
2019-11-25T15:55:57.2084501Z      |
2019-11-25T15:55:57.2084501Z      |
2019-11-25T15:55:57.2084831Z 238  | / macro_rules! int_impl {
2019-11-25T15:55:57.2085934Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-11-25T15:55:57.2086330Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-11-25T15:55:57.2086752Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-11-25T15:55:57.2087003Z ...    |
2019-11-25T15:55:57.2087375Z 2155 | |             pub const fn from_be_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
2019-11-25T15:55:57.2088007Z ...    |
2019-11-25T15:55:57.2088310Z 2236 | |     }
2019-11-25T15:55:57.2088595Z 2237 | | }
2019-11-25T15:55:57.2088595Z 2237 | | }
2019-11-25T15:55:57.2089053Z      | |_- in this expansion of `int_impl!`
2019-11-25T15:55:57.2089443Z ...
2019-11-25T15:55:57.2089877Z 2247 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
2019-11-25T15:55:57.2090222Z 2248 | |         "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
2019-11-25T15:55:57.2090765Z 
2019-11-25T15:55:58.4638946Z warning: Constant evaluating a complex constant, this might take some time
2019-11-25T15:55:58.4640229Z     --> src/libcore/num/mod.rs:2188:52
2019-11-25T15:55:58.4641129Z      |
2019-11-25T15:55:58.4641129Z      |
2019-11-25T15:55:58.4641652Z 238  | / macro_rules! int_impl {
2019-11-25T15:55:58.4642068Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-11-25T15:55:58.4642475Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-11-25T15:55:58.4642848Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-11-25T15:55:58.4643098Z ...    |
2019-11-25T15:55:58.4643496Z 2188 | |             pub const fn from_le_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
2019-11-25T15:55:58.4644112Z ...    |
2019-11-25T15:55:58.4644398Z 2236 | |     }
2019-11-25T15:55:58.4644694Z 2237 | | }
2019-11-25T15:55:58.4644694Z 2237 | | }
2019-11-25T15:55:58.4645278Z      | |_- in this expansion of `int_impl!`
2019-11-25T15:55:58.4645523Z ...
2019-11-25T15:55:58.4645915Z 2247 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
2019-11-25T15:55:58.4646302Z 2248 | |         "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
2019-11-25T15:55:58.4646720Z 
2019-11-25T15:55:59.7313744Z warning: Constant evaluating a complex constant, this might take some time
2019-11-25T15:55:59.7314494Z     --> src/libcore/num/mod.rs:2231:52
2019-11-25T15:55:59.7314960Z      |
2019-11-25T15:55:59.7314960Z      |
2019-11-25T15:55:59.7315492Z 238  | / macro_rules! int_impl {
2019-11-25T15:55:59.7317212Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-11-25T15:55:59.7317923Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-11-25T15:55:59.7318647Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-11-25T15:55:59.7319259Z ...    |
2019-11-25T15:55:59.7319862Z 2231 | |             pub const fn from_ne_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
2019-11-25T15:55:59.7320875Z ...    |
2019-11-25T15:55:59.7321359Z 2236 | |     }
2019-11-25T15:55:59.7321851Z 2237 | | }
2019-11-25T15:55:59.7321851Z 2237 | | }
2019-11-25T15:55:59.7322354Z      | |_- in this expansion of `int_impl!`
2019-11-25T15:55:59.7322771Z ...
2019-11-25T15:55:59.7323398Z 2247 | /     int_impl! { i16, i16, u16, 16, -32768, 32767, "", "", 4, "-0x5ffd", "0x3a", "0x1234", "0x3412",
2019-11-25T15:55:59.7323943Z 2248 | |         "0x2c48", "[0x34, 0x12]", "[0x12, 0x34]", "", "" }
2019-11-25T15:55:59.7324725Z 
2019-11-25T15:56:01.0089900Z warning: Constant evaluating a complex constant, this might take some time
2019-11-25T15:56:01.0090274Z     --> src/libcore/num/mod.rs:2066:52
2019-11-25T15:56:01.0090539Z      |
2019-11-25T15:56:01.0090539Z      |
2019-11-25T15:56:01.0090843Z 238  | / macro_rules! int_impl {
2019-11-25T15:56:01.0091228Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-11-25T15:56:01.0091612Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-11-25T15:56:01.0091964Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-11-25T15:56:01.0092206Z ...    |
2019-11-25T15:56:01.0092549Z 2066 | |             pub const fn to_be_bytes(self) -> [u8; mem::size_of::<Self>()] {
2019-11-25T15:56:01.0093144Z ...    |
2019-11-25T15:56:01.0093417Z 2236 | |     }
2019-11-25T15:56:01.0093881Z 2237 | | }
2019-11-25T15:56:01.0093881Z 2237 | | }
2019-11-25T15:56:01.0094227Z      | |_- in this expansion of `int_impl!`
2019-11-25T15:56:01.0094448Z ...
2019-11-25T15:56:01.0094825Z 2253 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
2019-11-25T15:56:01.0095327Z 2254 | |         "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
2019-11-25T15:56:01.0095670Z 2255 | |         "[0x12, 0x34, 0x56, 0x78]", "", "" }
2019-11-25T15:56:01.0096052Z 
2019-11-25T15:56:02.2808485Z warning: Constant evaluating a complex constant, this might take some time
2019-11-25T15:56:02.2809720Z     --> src/libcore/num/mod.rs:2086:52
2019-11-25T15:56:02.2810191Z      |
2019-11-25T15:56:02.2810191Z      |
2019-11-25T15:56:02.2810713Z 238  | / macro_rules! int_impl {
2019-11-25T15:56:02.2811365Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-11-25T15:56:02.2811953Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-11-25T15:56:02.2812523Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-11-25T15:56:02.2812970Z ...    |
2019-11-25T15:56:02.2813582Z 2086 | |             pub const fn to_le_bytes(self) -> [u8; mem::size_of::<Self>()] {
2019-11-25T15:56:02.2814636Z ...    |
2019-11-25T15:56:02.2815313Z 2236 | |     }
2019-11-25T15:56:02.2815827Z 2237 | | }
2019-11-25T15:56:02.2815827Z 2237 | | }
2019-11-25T15:56:02.2816407Z      | |_- in this expansion of `int_impl!`
2019-11-25T15:56:02.2816840Z ...
2019-11-25T15:56:02.2817442Z 2253 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
2019-11-25T15:56:02.2818043Z 2254 | |         "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
2019-11-25T15:56:02.2818639Z 2255 | |         "[0x12, 0x34, 0x56, 0x78]", "", "" }
2019-11-25T15:56:02.2819926Z 
2019-11-25T15:56:03.5314259Z warning: Constant evaluating a complex constant, this might take some time
2019-11-25T15:56:03.5314851Z     --> src/libcore/num/mod.rs:2121:52
2019-11-25T15:56:03.5315076Z      |
2019-11-25T15:56:03.5315076Z      |
2019-11-25T15:56:03.5315369Z 238  | / macro_rules! int_impl {
2019-11-25T15:56:03.5315888Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-11-25T15:56:03.5316242Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-11-25T15:56:03.5316580Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-11-25T15:56:03.5316815Z ...    |
2019-11-25T15:56:03.5317146Z 2121 | |             pub const fn to_ne_bytes(self) -> [u8; mem::size_of::<Self>()] {
2019-11-25T15:56:03.5317702Z ...    |
2019-11-25T15:56:03.5317956Z 2236 | |     }
2019-11-25T15:56:03.5318239Z 2237 | | }
2019-11-25T15:56:03.5318239Z 2237 | | }
2019-11-25T15:56:03.5318519Z      | |_- in this expansion of `int_impl!`
2019-11-25T15:56:03.5318705Z ...
2019-11-25T15:56:03.5319080Z 2253 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
2019-11-25T15:56:03.5319435Z 2254 | |         "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
2019-11-25T15:56:03.5319764Z 2255 | |         "[0x12, 0x34, 0x56, 0x78]", "", "" }
2019-11-25T15:56:03.5325047Z 
2019-11-25T15:56:04.7859660Z warning: Constant evaluating a complex constant, this might take some time
2019-11-25T15:56:04.7860864Z     --> src/libcore/num/mod.rs:2155:52
2019-11-25T15:56:04.7861358Z      |
2019-11-25T15:56:04.7861358Z      |
2019-11-25T15:56:04.7862149Z 238  | / macro_rules! int_impl {
2019-11-25T15:56:04.7862550Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-11-25T15:56:04.7862982Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-11-25T15:56:04.7863342Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-11-25T15:56:04.7863608Z ...    |
2019-11-25T15:56:04.7864169Z 2155 | |             pub const fn from_be_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
2019-11-25T15:56:04.7864847Z ...    |
2019-11-25T15:56:04.7865838Z 2236 | |     }
2019-11-25T15:56:04.7866336Z 2237 | | }
2019-11-25T15:56:04.7866336Z 2237 | | }
2019-11-25T15:56:04.7866675Z      | |_- in this expansion of `int_impl!`
2019-11-25T15:56:04.7866881Z ...
2019-11-25T15:56:04.7867282Z 2253 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
2019-11-25T15:56:04.7867699Z 2254 | |         "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
2019-11-25T15:56:04.7868039Z 2255 | |         "[0x12, 0x34, 0x56, 0x78]", "", "" }
2019-11-25T15:56:04.7873368Z 
2019-11-25T15:56:06.1128168Z warning: Constant evaluating a complex constant, this might take some time
2019-11-25T15:56:06.1130019Z     --> src/libcore/num/mod.rs:2188:52
2019-11-25T15:56:06.1130641Z      |
2019-11-25T15:56:06.1130641Z      |
2019-11-25T15:56:06.1131632Z 238  | / macro_rules! int_impl {
2019-11-25T15:56:06.1132351Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-11-25T15:56:06.1133032Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-11-25T15:56:06.1133726Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-11-25T15:56:06.1134273Z ...    |
2019-11-25T15:56:06.1134978Z 2188 | |             pub const fn from_le_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
2019-11-25T15:56:06.1136229Z ...    |
2019-11-25T15:56:06.1136815Z 2236 | |     }
2019-11-25T15:56:06.1137426Z 2237 | | }
2019-11-25T15:56:06.1137426Z 2237 | | }
2019-11-25T15:56:06.1138436Z      | |_- in this expansion of `int_impl!`
2019-11-25T15:56:06.1138979Z ...
2019-11-25T15:56:06.1139720Z 2253 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
2019-11-25T15:56:06.1140632Z 2254 | |         "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
2019-11-25T15:56:06.1229480Z 2255 | |         "[0x12, 0x34, 0x56, 0x78]", "", "" }
2019-11-25T15:56:06.1236018Z 
2019-11-25T15:56:07.4066736Z warning: Constant evaluating a complex constant, this might take some time
2019-11-25T15:56:07.4067096Z     --> src/libcore/num/mod.rs:2231:52
2019-11-25T15:56:07.4067370Z      |
2019-11-25T15:56:07.4067370Z      |
2019-11-25T15:56:07.4067698Z 238  | / macro_rules! int_impl {
2019-11-25T15:56:07.4068061Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-11-25T15:56:07.4068444Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-11-25T15:56:07.4068962Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-11-25T15:56:07.4069415Z ...    |
2019-11-25T15:56:07.4069785Z 2231 | |             pub const fn from_ne_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
2019-11-25T15:56:07.4070405Z ...    |
2019-11-25T15:56:07.4070686Z 2236 | |     }
2019-11-25T15:56:07.4070974Z 2237 | | }
2019-11-25T15:56:07.4070974Z 2237 | | }
2019-11-25T15:56:07.4071628Z      | |_- in this expansion of `int_impl!`
2019-11-25T15:56:07.4071820Z ...
2019-11-25T15:56:07.4072383Z 2253 | /     int_impl! { i32, i32, u32, 32, -2147483648, 2147483647, "", "", 8, "0x10000b3", "0xb301",
2019-11-25T15:56:07.4072770Z 2254 | |         "0x12345678", "0x78563412", "0x1e6a2c48", "[0x78, 0x56, 0x34, 0x12]",
2019-11-25T15:56:07.4073096Z 2255 | |         "[0x12, 0x34, 0x56, 0x78]", "", "" }
2019-11-25T15:56:07.4078077Z 
2019-11-25T15:56:08.6517701Z warning: Constant evaluating a complex constant, this might take some time
2019-11-25T15:56:08.6519115Z     --> src/libcore/num/mod.rs:2066:52
2019-11-25T15:56:08.6519715Z      |
2019-11-25T15:56:08.6519715Z      |
2019-11-25T15:56:08.6520378Z 238  | / macro_rules! int_impl {
2019-11-25T15:56:08.6521333Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-11-25T15:56:08.6522118Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-11-25T15:56:08.6523010Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-11-25T15:56:08.6523585Z ...    |
2019-11-25T15:56:08.6524296Z 2066 | |             pub const fn to_be_bytes(self) -> [u8; mem::size_of::<Self>()] {
2019-11-25T15:56:08.6525830Z ...    |
2019-11-25T15:56:08.6526772Z 2236 | |     }
2019-11-25T15:56:08.6527375Z 2237 | | }
2019-11-25T15:56:08.6527375Z 2237 | | }
2019-11-25T15:56:08.6528040Z      | |_- in this expansion of `int_impl!`
2019-11-25T15:56:08.6528559Z ...
2019-11-25T15:56:08.6529392Z 2260 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
2019-11-25T15:56:08.6530100Z 2261 | |          "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
2019-11-25T15:56:08.6531015Z 2262 | |          "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2019-11-25T15:56:08.6532138Z 2263 | |          "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
2019-11-25T15:56:08.6536542Z 
2019-11-25T15:56:09.9451051Z warning: Constant evaluating a complex constant, this might take some time
2019-11-25T15:56:09.9451977Z     --> src/libcore/num/mod.rs:2086:52
2019-11-25T15:56:09.9452447Z      |
2019-11-25T15:56:09.9452447Z      |
2019-11-25T15:56:09.9453117Z 238  | / macro_rules! int_impl {
2019-11-25T15:56:09.9453647Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-11-25T15:56:09.9454194Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-11-25T15:56:09.9454694Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-11-25T15:56:09.9455110Z ...    |
2019-11-25T15:56:09.9455591Z 2086 | |             pub const fn to_le_bytes(self) -> [u8; mem::size_of::<Self>()] {
2019-11-25T15:56:09.9456721Z ...    |
2019-11-25T15:56:09.9457171Z 2236 | |     }
2019-11-25T15:56:09.9457753Z 2237 | | }
2019-11-25T15:56:09.9457753Z 2237 | | }
2019-11-25T15:56:09.9458201Z      | |_- in this expansion of `int_impl!`
2019-11-25T15:56:09.9458579Z ...
2019-11-25T15:56:09.9459081Z 2260 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
2019-11-25T15:56:09.9459620Z 2261 | |          "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
2019-11-25T15:56:09.9460133Z 2262 | |          "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2019-11-25T15:56:09.9460649Z 2263 | |          "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
2019-11-25T15:56:09.9464848Z 
2019-11-25T15:56:11.2095343Z warning: Constant evaluating a complex constant, this might take some time
2019-11-25T15:56:11.2095701Z     --> src/libcore/num/mod.rs:2121:52
2019-11-25T15:56:11.2096253Z      |
2019-11-25T15:56:11.2096253Z      |
2019-11-25T15:56:11.2096586Z 238  | / macro_rules! int_impl {
2019-11-25T15:56:11.2096972Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-11-25T15:56:11.2097406Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-11-25T15:56:11.2097758Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-11-25T15:56:11.2098006Z ...    |
2019-11-25T15:56:11.2098398Z 2121 | |             pub const fn to_ne_bytes(self) -> [u8; mem::size_of::<Self>()] {
2019-11-25T15:56:11.2099015Z ...    |
2019-11-25T15:56:11.2099314Z 2236 | |     }
2019-11-25T15:56:11.2099806Z 2237 | | }
2019-11-25T15:56:11.2099806Z 2237 | | }
2019-11-25T15:56:11.2100356Z      | |_- in this expansion of `int_impl!`
2019-11-25T15:56:11.2100563Z ...
2019-11-25T15:56:11.2101320Z 2260 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
2019-11-25T15:56:11.2101785Z 2261 | |          "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
2019-11-25T15:56:11.2102164Z 2262 | |          "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2019-11-25T15:56:11.2102693Z 2263 | |          "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
2019-11-25T15:56:11.2107419Z 
2019-11-25T15:56:12.4738075Z warning: Constant evaluating a complex constant, this might take some time
2019-11-25T15:56:12.4739358Z     --> src/libcore/num/mod.rs:2155:52
2019-11-25T15:56:12.4739894Z      |
2019-11-25T15:56:12.4739894Z      |
2019-11-25T15:56:12.4740437Z 238  | / macro_rules! int_impl {
2019-11-25T15:56:12.4741071Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-11-25T15:56:12.4741806Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-11-25T15:56:12.4742365Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-11-25T15:56:12.4742857Z ...    |
2019-11-25T15:56:12.4743453Z 2155 | |             pub const fn from_be_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
2019-11-25T15:56:12.4744500Z ...    |
2019-11-25T15:56:12.4745000Z 2236 | |     }
2019-11-25T15:56:12.4745528Z 2237 | | }
2019-11-25T15:56:12.4745528Z 2237 | | }
2019-11-25T15:56:12.4746045Z      | |_- in this expansion of `int_impl!`
2019-11-25T15:56:12.4746502Z ...
2019-11-25T15:56:12.4747115Z 2260 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
2019-11-25T15:56:12.4747728Z 2261 | |          "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
2019-11-25T15:56:12.4748370Z 2262 | |          "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2019-11-25T15:56:12.4748951Z 2263 | |          "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
2019-11-25T15:56:12.4754546Z 
2019-11-25T15:56:13.7299275Z warning: Constant evaluating a complex constant, this might take some time
2019-11-25T15:56:13.7300519Z     --> src/libcore/num/mod.rs:2188:52
2019-11-25T15:56:13.7301239Z      |
2019-11-25T15:56:13.7301239Z      |
2019-11-25T15:56:13.7302626Z 238  | / macro_rules! int_impl {
2019-11-25T15:56:13.7303256Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-11-25T15:56:13.7303832Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-11-25T15:56:13.7304664Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-11-25T15:56:13.7305263Z ...    |
2019-11-25T15:56:13.7305804Z 2188 | |             pub const fn from_le_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
2019-11-25T15:56:13.7306780Z ...    |
2019-11-25T15:56:13.7307284Z 2236 | |     }
2019-11-25T15:56:13.7307738Z 2237 | | }
2019-11-25T15:56:13.7307738Z 2237 | | }
2019-11-25T15:56:13.7308249Z      | |_- in this expansion of `int_impl!`
2019-11-25T15:56:13.7308644Z ...
2019-11-25T15:56:13.7309192Z 2260 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
2019-11-25T15:56:13.7309986Z 2261 | |          "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
2019-11-25T15:56:13.7310551Z 2262 | |          "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2019-11-25T15:56:13.7311098Z 2263 | |          "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
2019-11-25T15:56:13.7315855Z 
2019-11-25T15:56:14.9794141Z warning: Constant evaluating a complex constant, this might take some time
2019-11-25T15:56:14.9795721Z     --> src/libcore/num/mod.rs:2231:52
2019-11-25T15:56:14.9796407Z      |
2019-11-25T15:56:14.9796407Z      |
2019-11-25T15:56:14.9797024Z 238  | / macro_rules! int_impl {
2019-11-25T15:56:14.9797693Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-11-25T15:56:14.9798582Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-11-25T15:56:14.9799289Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-11-25T15:56:14.9799870Z ...    |
2019-11-25T15:56:14.9800709Z 2231 | |             pub const fn from_ne_bytes(bytes: [u8; mem::size_of::<Self>()]) -> Self {
2019-11-25T15:56:14.9802125Z ...    |
2019-11-25T15:56:14.9802819Z 2236 | |     }
2019-11-25T15:56:14.9803485Z 2237 | | }
2019-11-25T15:56:14.9803485Z 2237 | | }
2019-11-25T15:56:14.9804152Z      | |_- in this expansion of `int_impl!`
2019-11-25T15:56:14.9805024Z ...
2019-11-25T15:56:14.9805945Z 2260 | /     int_impl! { i64, i64, u64, 64, -9223372036854775808, 9223372036854775807, "", "", 12,
2019-11-25T15:56:14.9806686Z 2261 | |          "0xaa00000000006e1", "0x6e10aa", "0x1234567890123456", "0x5634129078563412",
2019-11-25T15:56:14.9807389Z 2262 | |          "0x6a2c48091e6a2c48", "[0x56, 0x34, 0x12, 0x90, 0x78, 0x56, 0x34, 0x12]",
2019-11-25T15:56:14.9808097Z 2263 | |          "[0x12, 0x34, 0x56, 0x78, 0x90, 0x12, 0x34, 0x56]", "", "" }
2019-11-25T15:56:14.9812222Z 
2019-11-25T15:56:16.2178023Z warning: Constant evaluating a complex constant, this might take some time
2019-11-25T15:56:16.2179286Z     --> src/libcore/num/mod.rs:2066:52
2019-11-25T15:56:16.2179864Z      |
2019-11-25T15:56:16.2179864Z      |
2019-11-25T15:56:16.2180501Z 238  | / macro_rules! int_impl {
2019-11-25T15:56:16.2181212Z 239  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-11-25T15:56:16.2182052Z 240  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-11-25T15:56:16.2182694Z 241  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-11-25T15:56:16.2183279Z ...    |
2019-11-25T15:56:16.2183923Z 2066 | |             pub const fn to_be_bytes(self) -> [u8; mem::size_of::<Self>()] {
---
2019-11-25T15:57:15.8242635Z   local time: Mon Nov 25 15:57:15 UTC 2019
2019-11-25T15:57:16.0882786Z   network time: Mon, 25 Nov 2019 15:57:16 GMT
2019-11-25T15:57:16.0887272Z == end clock drift check ==
2019-11-25T15:57:16.5879515Z 
2019-11-25T15:57:16.5981460Z ##[error]Bash exited with code '1'.
2019-11-25T15:57:16.6039298Z ##[section]Starting: Checkout
2019-11-25T15:57:16.6041002Z ==============================================================================
2019-11-25T15:57:16.6041057Z Task         : Get sources
2019-11-25T15:57:16.6041120Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
