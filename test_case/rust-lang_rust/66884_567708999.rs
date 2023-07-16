plain
2019-12-19T22:31:22.6214528Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-19T22:31:22.6388077Z ##[command]git config gc.auto 0
2019-12-19T22:31:22.6468311Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-19T22:31:22.6530234Z ##[command]git config --get-all http.proxy
2019-12-19T22:31:22.6678700Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66884/merge:refs/remotes/pull/66884/merge
---
2019-12-19T22:36:14.1877335Z ...
2019-12-19T22:36:14.1877707Z 1625 | }
2019-12-19T22:36:14.1877992Z      |   ^
2019-12-19T22:36:14.1878033Z 
2019-12-19T22:36:14.1923990Z error: expected one of `async`, `const`, `crate`, `default`, `extern`, `fn`, `pub`, `type`, or `unsafe`, found keyword `impl`
2019-12-19T22:36:14.1924530Z      |
2019-12-19T22:36:14.1924815Z 1211 | #[stable(feature = "rust1", since = "1.0.0")]
2019-12-19T22:36:14.1925130Z      |                                              - expected one of 9 possible tokens
2019-12-19T22:36:14.1925130Z      |                                              - expected one of 9 possible tokens
2019-12-19T22:36:14.1925382Z 1212 | impl<T: Clone> Clone for Option<T> {
2019-12-19T22:36:14.1925893Z 
2019-12-19T22:36:16.6378345Z error[E0412]: cannot find type `IntoIter` in module `crate::option`
2019-12-19T22:36:16.6378760Z    --> src/libcore/iter/sources.rs:297:27
2019-12-19T22:36:16.6379005Z     |
---
2019-12-19T22:36:16.6603436Z 
2019-12-19T22:36:16.6712919Z error[E0422]: cannot find struct, variant or union type `Iter` in this scope
2019-12-19T22:36:16.6713189Z    --> src/libcore/option.rs:582:9
2019-12-19T22:36:16.6713402Z     |
2019-12-19T22:36:16.6713644Z 582 |         Iter { inner: Item { opt: self.as_ref() } }
2019-12-19T22:36:16.6714123Z     |
2019-12-19T22:36:16.6714528Z help: possible candidates are found in other modules, you can import them into scope
2019-12-19T22:36:16.6714753Z     |
2019-12-19T22:36:16.6714991Z 140 | use crate::result::Iter;
2019-12-19T22:36:16.6714991Z 140 | use crate::result::Iter;
2019-12-19T22:36:16.6715168Z     |
2019-12-19T22:36:16.6715384Z 140 | use crate::slice::Iter;
2019-12-19T22:36:16.6715578Z     |
2019-12-19T22:36:16.6715606Z 
2019-12-19T22:36:16.6800271Z error[E0422]: cannot find struct, variant or union type `Item` in this scope
2019-12-19T22:36:16.6800810Z    --> src/libcore/option.rs:582:23
2019-12-19T22:36:16.6801000Z     |
2019-12-19T22:36:16.6801242Z 582 |         Iter { inner: Item { opt: self.as_ref() } }
2019-12-19T22:36:16.6801773Z 
2019-12-19T22:36:16.6920133Z error[E0412]: cannot find type `IterMut` in this scope
2019-12-19T22:36:16.6920666Z    --> src/libcore/option.rs:602:35
2019-12-19T22:36:16.6920854Z     |
2019-12-19T22:36:16.6920854Z     |
2019-12-19T22:36:16.6921098Z 602 |     pub fn iter_mut(&mut self) -> IterMut<'_, T> {
2019-12-19T22:36:16.6921581Z     |
2019-12-19T22:36:16.6921837Z help: possible candidates are found in other modules, you can import them into scope
2019-12-19T22:36:16.6922041Z     |
2019-12-19T22:36:16.6922265Z 140 | use crate::result::IterMut;
2019-12-19T22:36:16.6922265Z 140 | use crate::result::IterMut;
2019-12-19T22:36:16.6922441Z     |
2019-12-19T22:36:16.6922676Z 140 | use crate::slice::IterMut;
2019-12-19T22:36:16.6922855Z     |
2019-12-19T22:36:16.6922883Z 
2019-12-19T22:36:16.7045445Z error[E0422]: cannot find struct, variant or union type `IterMut` in this scope
2019-12-19T22:36:16.7045733Z    --> src/libcore/option.rs:603:9
2019-12-19T22:36:16.7045929Z     |
2019-12-19T22:36:16.7046206Z 603 |         IterMut { inner: Item { opt: self.as_mut() } }
2019-12-19T22:36:16.7047486Z     |
2019-12-19T22:36:16.7047817Z help: possible candidates are found in other modules, you can import them into scope
2019-12-19T22:36:16.7048055Z     |
2019-12-19T22:36:16.7048343Z 140 | use crate::result::IterMut;
2019-12-19T22:36:16.7048343Z 140 | use crate::result::IterMut;
2019-12-19T22:36:16.7048590Z     |
2019-12-19T22:36:16.7048878Z 140 | use crate::slice::IterMut;
2019-12-19T22:36:16.7049110Z     |
2019-12-19T22:36:16.7049147Z 
2019-12-19T22:36:16.7140099Z error[E0422]: cannot find struct, variant or union type `Item` in this scope
2019-12-19T22:36:16.7140621Z    --> src/libcore/option.rs:603:26
2019-12-19T22:36:16.7141020Z     |
2019-12-19T22:36:16.7141417Z 603 |         IterMut { inner: Item { opt: self.as_mut() } }
2019-12-19T22:36:16.7141773Z 
2019-12-19T22:36:16.7232974Z error[E0425]: cannot find function `expect_none_failed` in this scope
2019-12-19T22:36:16.7233255Z     --> src/libcore/option.rs:1030:13
2019-12-19T22:36:16.7233630Z      |
2019-12-19T22:36:16.7233630Z      |
2019-12-19T22:36:16.7233924Z 1030 |             expect_none_failed(msg, &val);
2019-12-19T22:36:16.7234429Z 
2019-12-19T22:36:16.7328013Z error[E0425]: cannot find function `expect_none_failed` in this scope
2019-12-19T22:36:16.7328361Z     --> src/libcore/option.rs:1072:13
2019-12-19T22:36:16.7328603Z      |
2019-12-19T22:36:16.7328603Z      |
2019-12-19T22:36:16.7328993Z 1072 |             expect_none_failed("called `Option::unwrap_none()` on a `Some` value", &val);
2019-12-19T22:36:16.7329391Z 
2019-12-19T22:36:16.7455971Z error[E0412]: cannot find type `IntoIter` in module `option`
2019-12-19T22:36:16.7456282Z     --> src/libcore/str/mod.rs:4479:25
2019-12-19T22:36:16.7456524Z      |
2019-12-19T22:36:16.7456524Z      |
2019-12-19T22:36:16.7457402Z 4479 |         Flatten<option::IntoIter<char::EscapeDebug>>,
2019-12-19T22:36:16.7458084Z      |
2019-12-19T22:36:16.7458398Z help: possible candidates are found in other modules, you can import them into scope
2019-12-19T22:36:16.7458632Z      |
2019-12-19T22:36:16.7458942Z 10   | use crate::array::iter::IntoIter;
2019-12-19T22:36:16.7458942Z 10   | use crate::array::iter::IntoIter;
2019-12-19T22:36:16.7459177Z      |
2019-12-19T22:36:16.7459466Z 10   | use crate::result::IntoIter;
2019-12-19T22:36:16.7459732Z      |
2019-12-19T22:36:16.7459769Z 
2019-12-19T22:36:17.2893184Z error: unused imports: `FromIterator`, `FusedIterator`, `TrustedLen`
2019-12-19T22:36:17.2893912Z     |
2019-12-19T22:36:17.2893912Z     |
2019-12-19T22:36:17.2894169Z 140 | use crate::iter::{FromIterator, FusedIterator, TrustedLen};
2019-12-19T22:36:17.2894698Z     |
2019-12-19T22:36:17.2894957Z     = note: `-D unused-imports` implied by `-D warnings`
2019-12-19T22:36:17.2894996Z 
2019-12-19T22:36:17.2944662Z error: unused imports: `convert`, `self`
2019-12-19T22:36:17.2944662Z error: unused imports: `convert`, `self`
2019-12-19T22:36:17.2945098Z    --> src/libcore/option.rs:143:5
2019-12-19T22:36:17.2945332Z     |
2019-12-19T22:36:17.2945563Z 143 |     convert, fmt, hint, mem,
2019-12-19T22:36:17.2945798Z     |     ^^^^^^^
2019-12-19T22:36:17.2946036Z 144 |     ops::{self, Deref, DerefMut},
2019-12-19T22:36:17.2946287Z 
2019-12-19T22:36:17.9186272Z error[E0547]: missing 'issue'
2019-12-19T22:36:17.9186598Z    --> src/libcore/num/mod.rs:73:17
2019-12-19T22:36:17.9186805Z     |
2019-12-19T22:36:17.9186805Z     |
2019-12-19T22:36:17.9187499Z 35  | / macro_rules! nonzero_integers {
2019-12-19T22:36:17.9187830Z 36  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
2019-12-19T22:36:17.9188388Z 38  | |             doc_comment! {
2019-12-19T22:36:17.9188599Z ...   |
2019-12-19T22:36:17.9188914Z 73  | |                 #[rustc_const_unstable(feature = "const_int_nonzero")]
2019-12-19T22:36:17.9189211Z     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-19T22:36:17.9189211Z     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-19T22:36:17.9189457Z ...   |
2019-12-19T22:36:17.9189715Z 104 | |     }
2019-12-19T22:36:17.9189960Z 105 | | }
2019-12-19T22:36:17.9190252Z     | |_- in this expansion of `nonzero_integers!`
2019-12-19T22:36:17.9190463Z 106 | 
2019-12-19T22:36:17.9190719Z 107 | / nonzero_integers! {
2019-12-19T22:36:17.9191035Z 108 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
2019-12-19T22:36:17.9191356Z 109 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
2019-12-19T22:36:17.9191681Z 110 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
2019-12-19T22:36:17.9192078Z ...   |
2019-12-19T22:36:17.9192392Z 119 | |     #[stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIsize(isize);
2019-12-19T22:36:17.9192940Z     | |_- in this macro invocation
2019-12-19T22:36:17.9192997Z 
2019-12-19T22:36:17.9938983Z error[E0547]: missing 'issue'
2019-12-19T22:36:17.9939355Z     --> src/libcore/num/mod.rs:648:13
2019-12-19T22:36:17.9939355Z     --> src/libcore/num/mod.rs:648:13
2019-12-19T22:36:17.9939818Z      |
2019-12-19T22:36:17.9940271Z 243  | / macro_rules! int_impl {
2019-12-19T22:36:17.9940860Z 244  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-19T22:36:17.9941395Z 245  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-19T22:36:17.9941698Z 246  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-19T22:36:17.9942678Z 648  | |             #[rustc_const_unstable(feature = "const_int_checked")]
2019-12-19T22:36:17.9942984Z      | |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-19T22:36:17.9943194Z ...    |
2019-12-19T22:36:17.9943436Z 2300 | |     }
2019-12-19T22:36:17.9943436Z 2300 | |     }
2019-12-19T22:36:17.9943691Z 2301 | | }
2019-12-19T22:36:17.9943951Z      | |_- in this expansion of `int_impl!`
2019-12-19T22:36:17.9944125Z ...
2019-12-19T22:36:17.9944472Z 2305 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-12-19T22:36:17.9944746Z 2306 | |     "[0x12]", "[0x12]", "", "" }
2019-12-19T22:36:17.9945076Z 
2019-12-19T22:36:18.0347302Z error[E0547]: missing 'issue'
2019-12-19T22:36:18.0348956Z     --> src/libcore/num/mod.rs:673:13
2019-12-19T22:36:18.0349285Z      |
2019-12-19T22:36:18.0349285Z      |
2019-12-19T22:36:18.0349637Z 243  | / macro_rules! int_impl {
2019-12-19T22:36:18.0350334Z 244  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-19T22:36:18.0351029Z 245  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-19T22:36:18.0351347Z 246  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-19T22:36:18.0351923Z 673  | |             #[rustc_const_unstable(feature = "const_int_checked")]
2019-12-19T22:36:18.0352230Z      | |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-19T22:36:18.0352480Z ...    |
2019-12-19T22:36:18.0352888Z 2300 | |     }
2019-12-19T22:36:18.0352888Z 2300 | |     }
2019-12-19T22:36:18.0353197Z 2301 | | }
2019-12-19T22:36:18.0353463Z      | |_- in this expansion of `int_impl!`
2019-12-19T22:36:18.0353661Z ...
2019-12-19T22:36:18.0353980Z 2305 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-12-19T22:36:18.0354454Z 2306 | |     "[0x12]", "[0x12]", "", "" }
2019-12-19T22:36:18.0355092Z 
2019-12-19T22:36:18.0869560Z error[E0547]: missing 'issue'
2019-12-19T22:36:18.0873371Z     --> src/libcore/num/mod.rs:698:13
2019-12-19T22:36:18.0873634Z      |
2019-12-19T22:36:18.0873634Z      |
2019-12-19T22:36:18.0873887Z 243  | / macro_rules! int_impl {
2019-12-19T22:36:18.0874211Z 244  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-19T22:36:18.0874522Z 245  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-19T22:36:18.0874848Z 246  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-19T22:36:18.0875340Z 698  | |             #[rustc_const_unstable(feature = "const_int_checked")]
2019-12-19T22:36:18.0875631Z      | |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-19T22:36:18.0875834Z ...    |
2019-12-19T22:36:18.0876071Z 2300 | |     }
2019-12-19T22:36:18.0876071Z 2300 | |     }
2019-12-19T22:36:18.0876364Z 2301 | | }
2019-12-19T22:36:18.0876802Z      | |_- in this expansion of `int_impl!`
2019-12-19T22:36:18.0877444Z ...
2019-12-19T22:36:18.0877898Z 2305 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-12-19T22:36:18.0878240Z 2306 | |     "[0x12]", "[0x12]", "", "" }
2019-12-19T22:36:18.0878654Z 
2019-12-19T22:36:18.1713539Z error[E0547]: missing 'issue'
2019-12-19T22:36:18.1714799Z     --> src/libcore/num/mod.rs:724:13
2019-12-19T22:36:18.1715027Z      |
2019-12-19T22:36:18.1715027Z      |
2019-12-19T22:36:18.1715488Z 243  | / macro_rules! int_impl {
2019-12-19T22:36:18.1715897Z 244  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-19T22:36:18.1716244Z 245  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-19T22:36:18.1716566Z 246  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-19T22:36:18.1717779Z 724  | |             #[rustc_const_unstable(feature = "const_int_checked")]
2019-12-19T22:36:18.1718346Z      | |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-19T22:36:18.1718626Z ...    |
2019-12-19T22:36:18.1718938Z 2300 | |     }
2019-12-19T22:36:18.1718938Z 2300 | |     }
2019-12-19T22:36:18.1719254Z 2301 | | }
2019-12-19T22:36:18.1719586Z      | |_- in this expansion of `int_impl!`
2019-12-19T22:36:18.1719812Z ...
2019-12-19T22:36:18.1720227Z 2305 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-12-19T22:36:18.1720595Z 2306 | |     "[0x12]", "[0x12]", "", "" }
2019-12-19T22:36:18.1721299Z 
2019-12-19T22:36:18.2351884Z error[E0547]: missing 'issue'
2019-12-19T22:36:18.2353696Z     --> src/libcore/num/mod.rs:753:13
2019-12-19T22:36:18.2357659Z      |
2019-12-19T22:36:18.2357659Z      |
2019-12-19T22:36:18.2358100Z 243  | / macro_rules! int_impl {
2019-12-19T22:36:18.2358532Z 244  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-19T22:36:18.2358970Z 245  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-19T22:36:18.2359357Z 246  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-19T22:36:18.2359626Z ...    |
2019-12-19T22:36:18.2359993Z 753  | |             #[rustc_const_unstable(feature = "const_int_euclidean")]
2019-12-19T22:36:18.2360803Z ...    |
2019-12-19T22:36:18.2361039Z 2300 | |     }
2019-12-19T22:36:18.2361424Z 2301 | | }
2019-12-19T22:36:18.2361721Z      | |_- in this expansion of `int_impl!`
2019-12-19T22:36:18.2361721Z      | |_- in this expansion of `int_impl!`
2019-12-19T22:36:18.2361895Z ...
2019-12-19T22:36:18.2362221Z 2305 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-12-19T22:36:18.2362485Z 2306 | |     "[0x12]", "[0x12]", "", "" }
2019-12-19T22:36:18.2362809Z 
2019-12-19T22:36:18.3073686Z error[E0547]: missing 'issue'
2019-12-19T22:36:18.3077617Z     --> src/libcore/num/mod.rs:812:13
2019-12-19T22:36:18.3077920Z      |
2019-12-19T22:36:18.3077920Z      |
2019-12-19T22:36:18.3078259Z 243  | / macro_rules! int_impl {
2019-12-19T22:36:18.3078695Z 244  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-19T22:36:18.3079111Z 245  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-19T22:36:18.3079478Z 246  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-19T22:36:18.3079787Z ...    |
2019-12-19T22:36:18.3080168Z 812  | |             #[rustc_const_unstable(feature = "const_int_euclidean")]
2019-12-19T22:36:18.3080988Z ...    |
2019-12-19T22:36:18.3081277Z 2300 | |     }
2019-12-19T22:36:18.3081919Z 2301 | | }
2019-12-19T22:36:18.3082369Z      | |_- in this expansion of `int_impl!`
2019-12-19T22:36:18.3082369Z      | |_- in this expansion of `int_impl!`
2019-12-19T22:36:18.3082980Z ...
2019-12-19T22:36:18.3083522Z 2305 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-12-19T22:36:18.3083832Z 2306 | |     "[0x12]", "[0x12]", "", "" }
2019-12-19T22:36:18.3084196Z 
2019-12-19T22:36:18.3831595Z error[E0547]: missing 'issue'
2019-12-19T22:36:18.3835746Z     --> src/libcore/num/mod.rs:838:13
2019-12-19T22:36:18.3836043Z      |
2019-12-19T22:36:18.3836043Z      |
2019-12-19T22:36:18.3836316Z 243  | / macro_rules! int_impl {
2019-12-19T22:36:18.3837389Z 244  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-19T22:36:18.3837886Z 245  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-19T22:36:18.3838250Z 246  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-19T22:36:18.3838901Z 838  | |             #[rustc_const_unstable(feature = "const_int_checked")]
2019-12-19T22:36:18.3839279Z      | |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-19T22:36:18.3839704Z ...    |
2019-12-19T22:36:18.3840019Z 2300 | |     }
2019-12-19T22:36:18.3840019Z 2300 | |     }
2019-12-19T22:36:18.3840341Z 2301 | | }
2019-12-19T22:36:18.3840671Z      | |_- in this expansion of `int_impl!`
2019-12-19T22:36:18.3840897Z ...
2019-12-19T22:36:18.3841462Z 2305 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-12-19T22:36:18.3841792Z 2306 | |     "[0x12]", "[0x12]", "", "" }
2019-12-19T22:36:18.3842211Z 
2019-12-19T22:36:18.4551734Z error[E0547]: missing 'issue'
2019-12-19T22:36:18.4552909Z     --> src/libcore/num/mod.rs:862:13
2019-12-19T22:36:18.4553310Z      |
2019-12-19T22:36:18.4553310Z      |
2019-12-19T22:36:18.4553991Z 243  | / macro_rules! int_impl {
2019-12-19T22:36:18.4554549Z 244  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-19T22:36:18.4555074Z 245  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-19T22:36:18.4555637Z 246  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-19T22:36:18.4556876Z 862  | |             #[rustc_const_unstable(feature = "const_int_checked")]
2019-12-19T22:36:18.4557959Z      | |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-19T22:36:18.4558449Z ...    |
2019-12-19T22:36:18.4558943Z 2300 | |     }
2019-12-19T22:36:18.4558943Z 2300 | |     }
2019-12-19T22:36:18.4559438Z 2301 | | }
2019-12-19T22:36:18.4560178Z      | |_- in this expansion of `int_impl!`
2019-12-19T22:36:18.4560784Z ...
2019-12-19T22:36:18.4561291Z 2305 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-12-19T22:36:18.4561723Z 2306 | |     "[0x12]", "[0x12]", "", "" }
2019-12-19T22:36:18.4562334Z 
2019-12-19T22:36:18.5309327Z error[E0547]: missing 'issue'
2019-12-19T22:36:18.5325747Z     --> src/libcore/num/mod.rs:887:13
2019-12-19T22:36:18.5326420Z      |
2019-12-19T22:36:18.5326420Z      |
2019-12-19T22:36:18.5326913Z 243  | / macro_rules! int_impl {
2019-12-19T22:36:18.5327820Z 244  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-19T22:36:18.5328246Z 245  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-19T22:36:18.5328644Z 246  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-19T22:36:18.5329440Z 887  | |             #[rustc_const_unstable(feature = "const_int_checked")]
2019-12-19T22:36:18.5329858Z      | |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-19T22:36:18.5330129Z ...    |
2019-12-19T22:36:18.5330462Z 2300 | |     }
2019-12-19T22:36:18.5330462Z 2300 | |     }
2019-12-19T22:36:18.5330897Z 2301 | | }
2019-12-19T22:36:18.5331189Z      | |_- in this expansion of `int_impl!`
2019-12-19T22:36:18.5331407Z ...
2019-12-19T22:36:18.5331755Z 2305 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-12-19T22:36:18.5332077Z 2306 | |     "[0x12]", "[0x12]", "", "" }
2019-12-19T22:36:18.5332453Z 
2019-12-19T22:36:18.5991556Z error[E0547]: missing 'issue'
2019-12-19T22:36:18.5992389Z     --> src/libcore/num/mod.rs:911:13
2019-12-19T22:36:18.5993121Z      |
2019-12-19T22:36:18.5993121Z      |
2019-12-19T22:36:18.5993394Z 243  | / macro_rules! int_impl {
2019-12-19T22:36:18.5993983Z 244  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-19T22:36:18.5994837Z 245  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-19T22:36:18.5995378Z 246  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-19T22:36:18.5996214Z 911  | |             #[rustc_const_unstable(feature = "const_int_checked")]
2019-12-19T22:36:18.5996825Z      | |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-19T22:36:18.5997763Z ...    |
2019-12-19T22:36:18.5998457Z 2300 | |     }
2019-12-19T22:36:18.5998457Z 2300 | |     }
2019-12-19T22:36:18.5998957Z 2301 | | }
2019-12-19T22:36:18.5999507Z      | |_- in this expansion of `int_impl!`
2019-12-19T22:36:18.5999910Z ...
2019-12-19T22:36:18.6000503Z 2305 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-12-19T22:36:18.6001137Z 2306 | |     "[0x12]", "[0x12]", "", "" }
2019-12-19T22:36:18.6001946Z 
2019-12-19T22:36:18.6709042Z error[E0547]: missing 'issue'
2019-12-19T22:36:18.6709845Z     --> src/libcore/num/mod.rs:1035:13
2019-12-19T22:36:18.6710430Z      |
2019-12-19T22:36:18.6710430Z      |
2019-12-19T22:36:18.6711343Z 243  | / macro_rules! int_impl {
2019-12-19T22:36:18.6711966Z 244  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-19T22:36:18.6712610Z 245  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-19T22:36:18.6713497Z 246  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-19T22:36:18.6714939Z 1035 | |             #[rustc_const_unstable(feature = "const_saturating_int_methods")]
2019-12-19T22:36:18.6715482Z      | |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-19T22:36:18.6715954Z ...    |
2019-12-19T22:36:18.6716429Z 2300 | |     }
2019-12-19T22:36:18.6716429Z 2300 | |     }
2019-12-19T22:36:18.6717757Z 2301 | | }
2019-12-19T22:36:18.6719451Z      | |_- in this expansion of `int_impl!`
2019-12-19T22:36:18.6721293Z ...
2019-12-19T22:36:18.6723444Z 2305 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-12-19T22:36:18.6726007Z 2306 | |     "[0x12]", "[0x12]", "", "" }
2019-12-19T22:36:18.6732050Z 
2019-12-19T22:36:18.7318644Z error[E0547]: missing 'issue'
2019-12-19T22:36:18.7318982Z     --> src/libcore/num/mod.rs:1064:13
2019-12-19T22:36:18.7319220Z      |
2019-12-19T22:36:18.7319220Z      |
2019-12-19T22:36:18.7319554Z 243  | / macro_rules! int_impl {
2019-12-19T22:36:18.7320200Z 244  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-19T22:36:18.7320641Z 245  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-19T22:36:18.7321297Z 246  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-19T22:36:18.7322198Z 1064 | |             #[rustc_const_unstable(feature = "const_saturating_int_methods")]
2019-12-19T22:36:18.7322533Z      | |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-19T22:36:18.7322788Z ...    |
2019-12-19T22:36:18.7323051Z 2300 | |     }
2019-12-19T22:36:18.7323051Z 2300 | |     }
2019-12-19T22:36:18.7323311Z 2301 | | }
2019-12-19T22:36:18.7323606Z      | |_- in this expansion of `int_impl!`
2019-12-19T22:36:18.7323801Z ...
2019-12-19T22:36:18.7324141Z 2305 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-12-19T22:36:18.7324452Z 2306 | |     "[0x12]", "[0x12]", "", "" }
2019-12-19T22:36:18.7330838Z 
2019-12-19T22:36:18.8112978Z error[E0547]: missing 'issue'
2019-12-19T22:36:18.8113243Z     --> src/libcore/num/mod.rs:1097:13
2019-12-19T22:36:18.8113432Z      |
2019-12-19T22:36:18.8113432Z      |
2019-12-19T22:36:18.8113707Z 243  | / macro_rules! int_impl {
2019-12-19T22:36:18.8114014Z 244  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-19T22:36:18.8114341Z 245  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-19T22:36:18.8114792Z 246  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-19T22:36:18.8115345Z 1097 | |             #[rustc_const_unstable(feature = "const_int_saturating")]
2019-12-19T22:36:18.8115625Z      | |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-19T22:36:18.8115827Z ...    |
2019-12-19T22:36:18.8116077Z 2300 | |     }
2019-12-19T22:36:18.8116077Z 2300 | |     }
2019-12-19T22:36:18.8116412Z 2301 | | }
2019-12-19T22:36:18.8116858Z      | |_- in this expansion of `int_impl!`
2019-12-19T22:36:18.8117468Z ...
2019-12-19T22:36:18.8117877Z 2305 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-12-19T22:36:18.8118239Z 2306 | |     "[0x12]", "[0x12]", "", "" }
2019-12-19T22:36:18.8123203Z 
2019-12-19T22:36:18.8750909Z error[E0547]: missing 'issue'
2019-12-19T22:36:18.8751199Z     --> src/libcore/num/mod.rs:1237:13
2019-12-19T22:36:18.8751399Z      |
2019-12-19T22:36:18.8751399Z      |
2019-12-19T22:36:18.8751684Z 243  | / macro_rules! int_impl {
2019-12-19T22:36:18.8751995Z 244  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-19T22:36:18.8752315Z 245  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-19T22:36:18.8752618Z 246  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-19T22:36:18.8753148Z 1237 | |             #[rustc_const_unstable(feature = "const_int_wrapping")]
2019-12-19T22:36:18.8753623Z      | |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-19T22:36:18.8754060Z ...    |
2019-12-19T22:36:18.8754429Z 2300 | |     }
2019-12-19T22:36:18.8754429Z 2300 | |     }
2019-12-19T22:36:18.8754719Z 2301 | | }
2019-12-19T22:36:18.8755032Z      | |_- in this expansion of `int_impl!`
2019-12-19T22:36:18.8755266Z ...
2019-12-19T22:36:18.8755646Z 2305 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-12-19T22:36:18.8756165Z 2306 | |     "[0x12]", "[0x12]", "", "" }
2019-12-19T22:36:18.8761586Z 
2019-12-19T22:36:18.9471081Z error[E0547]: missing 'issue'
2019-12-19T22:36:18.9471340Z     --> src/libcore/num/mod.rs:1267:13
2019-12-19T22:36:18.9471525Z      |
2019-12-19T22:36:18.9471525Z      |
2019-12-19T22:36:18.9471786Z 243  | / macro_rules! int_impl {
2019-12-19T22:36:18.9472091Z 244  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-19T22:36:18.9472597Z 245  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-19T22:36:18.9472905Z 246  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-19T22:36:18.9473108Z ...    |
2019-12-19T22:36:18.9473404Z 1267 | |             #[rustc_const_unstable(feature = "const_int_euclidean")]
2019-12-19T22:36:18.9473896Z ...    |
2019-12-19T22:36:18.9474149Z 2300 | |     }
2019-12-19T22:36:18.9474383Z 2301 | | }
2019-12-19T22:36:18.9474633Z      | |_- in this expansion of `int_impl!`
2019-12-19T22:36:18.9474633Z      | |_- in this expansion of `int_impl!`
2019-12-19T22:36:18.9474824Z ...
2019-12-19T22:36:18.9475127Z 2305 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-12-19T22:36:18.9475390Z 2306 | |     "[0x12]", "[0x12]", "", "" }
2019-12-19T22:36:18.9480466Z 
2019-12-19T22:36:19.0229705Z error[E0547]: missing 'issue'
2019-12-19T22:36:19.0230890Z     --> src/libcore/num/mod.rs:1298:13
2019-12-19T22:36:19.0231384Z      |
2019-12-19T22:36:19.0231384Z      |
2019-12-19T22:36:19.0231929Z 243  | / macro_rules! int_impl {
2019-12-19T22:36:19.0232522Z 244  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-19T22:36:19.0233104Z 245  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-19T22:36:19.0233824Z 246  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-19T22:36:19.0235096Z 1298 | |             #[rustc_const_unstable(feature = "const_int_wrapping")]
2019-12-19T22:36:19.0235630Z      | |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-19T22:36:19.0236102Z ...    |
2019-12-19T22:36:19.0236587Z 2300 | |     }
2019-12-19T22:36:19.0236587Z 2300 | |     }
2019-12-19T22:36:19.0238625Z 2301 | | }
2019-12-19T22:36:19.0239346Z      | |_- in this expansion of `int_impl!`
2019-12-19T22:36:19.0239987Z ...
2019-12-19T22:36:19.0240933Z 2305 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-12-19T22:36:19.0241385Z 2306 | |     "[0x12]", "[0x12]", "", "" }
2019-12-19T22:36:19.0245638Z 
2019-12-19T22:36:19.0989221Z error[E0547]: missing 'issue'
2019-12-19T22:36:19.0990245Z     --> src/libcore/num/mod.rs:1327:13
2019-12-19T22:36:19.0990934Z      |
2019-12-19T22:36:19.0990934Z      |
2019-12-19T22:36:19.0991467Z 243  | / macro_rules! int_impl {
2019-12-19T22:36:19.0992083Z 244  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-19T22:36:19.0992656Z 245  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-19T22:36:19.0993217Z 246  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-19T22:36:19.0993666Z ...    |
2019-12-19T22:36:19.0994218Z 1327 | |             #[rustc_const_unstable(feature = "const_int_euclidean")]
2019-12-19T22:36:19.0995242Z ...    |
2019-12-19T22:36:19.0995757Z 2300 | |     }
2019-12-19T22:36:19.0996267Z 2301 | | }
2019-12-19T22:36:19.0997330Z      | |_- in this expansion of `int_impl!`
2019-12-19T22:36:19.0997330Z      | |_- in this expansion of `int_impl!`
2019-12-19T22:36:19.0997927Z ...
2019-12-19T22:36:19.0998625Z 2305 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-12-19T22:36:19.0999303Z 2306 | |     "[0x12]", "[0x12]", "", "" }
2019-12-19T22:36:19.1003424Z 
2019-12-19T22:36:19.1591590Z error[E0547]: missing 'issue'
2019-12-19T22:36:19.1592862Z     --> src/libcore/num/mod.rs:1609:13
2019-12-19T22:36:19.1593387Z      |
2019-12-19T22:36:19.1593387Z      |
2019-12-19T22:36:19.1593892Z 243  | / macro_rules! int_impl {
2019-12-19T22:36:19.1594475Z 244  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-19T22:36:19.1595249Z 245  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-19T22:36:19.1596214Z 246  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-19T22:36:19.1598034Z 1609 | |             #[rustc_const_unstable(feature = "const_int_overflowing")]
2019-12-19T22:36:19.1598727Z      | |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-19T22:36:19.1599271Z ...    |
2019-12-19T22:36:19.1599880Z 2300 | |     }
2019-12-19T22:36:19.1599880Z 2300 | |     }
2019-12-19T22:36:19.1600516Z 2301 | | }
2019-12-19T22:36:19.1601225Z      | |_- in this expansion of `int_impl!`
2019-12-19T22:36:19.1601838Z ...
2019-12-19T22:36:19.1602891Z 2305 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-12-19T22:36:19.1603453Z 2306 | |     "[0x12]", "[0x12]", "", "" }
2019-12-19T22:36:19.1608200Z 
2019-12-19T22:36:19.2391589Z error[E0547]: missing 'issue'
2019-12-19T22:36:19.2391895Z     --> src/libcore/num/mod.rs:1644:13
2019-12-19T22:36:19.2392140Z      |
2019-12-19T22:36:19.2392140Z      |
2019-12-19T22:36:19.2392414Z 243  | / macro_rules! int_impl {
2019-12-19T22:36:19.2392747Z 244  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-19T22:36:19.2393101Z 245  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-19T22:36:19.2393396Z 246  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-19T22:36:19.2393646Z ...    |
2019-12-19T22:36:19.2394096Z 1644 | |             #[rustc_const_unstable(feature = "const_int_euclidean")]
2019-12-19T22:36:19.2394679Z ...    |
2019-12-19T22:36:19.2394927Z 2300 | |     }
2019-12-19T22:36:19.2395188Z 2301 | | }
2019-12-19T22:36:19.2395454Z      | |_- in this expansion of `int_impl!`
2019-12-19T22:36:19.2395454Z      | |_- in this expansion of `int_impl!`
2019-12-19T22:36:19.2395636Z ...
2019-12-19T22:36:19.2396105Z 2305 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-12-19T22:36:19.2396389Z 2306 | |     "[0x12]", "[0x12]", "", "" }
2019-12-19T22:36:19.2401618Z 
2019-12-19T22:36:19.3114297Z error[E0547]: missing 'issue'
2019-12-19T22:36:19.3114565Z     --> src/libcore/num/mod.rs:1679:13
2019-12-19T22:36:19.3114781Z      |
2019-12-19T22:36:19.3114781Z      |
2019-12-19T22:36:19.3115036Z 243  | / macro_rules! int_impl {
2019-12-19T22:36:19.3115394Z 244  | |     ($SelfT:ty, $ActualT:ident, $UnsignedT:ty, $BITS:expr, $Min:expr, $Max:expr, $Feature:expr,
2019-12-19T22:36:19.3115732Z 245  | |      $EndFeature:expr, $rot:expr, $rot_op:expr, $rot_result:expr, $swap_op:expr, $swapped:expr,
2019-12-19T22:36:19.3116010Z 246  | |      $reversed:expr, $le_bytes:expr, $be_bytes:expr,
2019-12-19T22:36:19.3116516Z 1679 | |             #[rustc_const_unstable(feature = "const_int_overflowing")]
2019-12-19T22:36:19.3117484Z      | |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-19T22:36:19.3117809Z ...    |
2019-12-19T22:36:19.3118124Z 2300 | |     }
2019-12-19T22:36:19.3118124Z 2300 | |     }
2019-12-19T22:36:19.3118447Z 2301 | | }
2019-12-19T22:36:19.3118771Z      | |_- in this expansion of `int_impl!`
2019-12-19T22:36:19.3118992Z ...
2019-12-19T22:36:19.3119410Z 2305 | /     int_impl! { i8, i8, u8, 8, -128, 127, "", "", 2, "-0x7e", "0xa", "0x12", "0x12", "0x48",
2019-12-19T22:36:19.3119751Z 2306 | |     "[0x12]", "[0x12]", "", "" }
2019-12-19T22:36:19.3124222Z 
2019-12-19T22:36:19.3832109Z error[E0547]: missing 'issue'
2019-12-19T22:36:19.3832393Z     --> src/libcore/num/mod.rs:1713:13
2019-12-19T22:36:19.3832619Z      |
2019-12-19T22:36:19.3832619Z      |
---
2019-12-19T22:36:36.9898843Z    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
2019-12-19T22:36:37.3497574Z error[E0277]: the trait bound `option::Option<T>: clone::Clone` is not satisfied
2019-12-19T22:36:37.3498002Z    --> src/libcore/option.rs:153:10
2019-12-19T22:36:37.3498279Z     |
2019-12-19T22:36:37.3498609Z 153 | #[derive(Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
2019-12-19T22:36:37.3499181Z     |          |
2019-12-19T22:36:37.3499523Z     |          the trait `clone::Clone` is not implemented for `option::Option<T>`
2019-12-19T22:36:37.3499825Z     |          in this macro invocation
2019-12-19T22:36:37.3500098Z     | 
2019-12-19T22:36:37.3500098Z     | 
2019-12-19T22:36:37.3500364Z    ::: src/libcore/marker.rs:373:1
2019-12-19T22:36:37.3501028Z     |
2019-12-19T22:36:37.3501371Z 373 | pub macro Copy($item:item) { /* compiler built-in */ }
2019-12-19T22:36:37.3501696Z     | ------------------------------------------------------ in this expansion of `#[derive(Copy)]`
2019-12-19T22:36:37.7040788Z    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
2019-12-19T22:36:38.1354265Z error: aborting due to 291 previous errors
2019-12-19T22:36:38.1355129Z 
2019-12-19T22:36:38.1359505Z Some errors have detailed explanations: E0277, E0412, E0422, E0425.
---
2019-12-19T22:36:38.3867437Z   local time: Thu Dec 19 22:36:38 UTC 2019
2019-12-19T22:36:38.5343238Z   network time: Thu, 19 Dec 2019 22:36:38 GMT
2019-12-19T22:36:38.5348836Z == end clock drift check ==
2019-12-19T22:36:47.1698194Z 
2019-12-19T22:36:47.1796837Z ##[error]Bash exited with code '1'.
2019-12-19T22:36:47.1824213Z ##[section]Starting: Checkout
2019-12-19T22:36:47.1825772Z ==============================================================================
2019-12-19T22:36:47.1825821Z Task         : Get sources
2019-12-19T22:36:47.1825879Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
