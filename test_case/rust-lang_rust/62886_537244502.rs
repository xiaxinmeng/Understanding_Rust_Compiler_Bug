plain
2019-10-01T21:35:09.6584322Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-01T21:35:09.6788755Z ##[command]git config gc.auto 0
2019-10-01T21:35:09.6861878Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-01T21:35:09.6925475Z ##[command]git config --get-all http.proxy
2019-10-01T21:35:09.7087036Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62886/merge:refs/remotes/pull/62886/merge
---
2019-10-01T21:41:18.3509518Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-01T21:41:18.3585747Z Checking std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-10-01T21:41:18.7542333Z    Compiling cc v1.0.35
2019-10-01T21:41:18.7554551Z     Checking core v0.0.0 (/checkout/src/libcore)
2019-10-01T21:41:27.1095772Z error[E0599]: no function or associated item named `unchecked_add` found for type `u8` in the current scope
2019-10-01T21:41:27.1096430Z    --> src/libcore/iter/range.rs:204:19
2019-10-01T21:41:27.1096944Z     |
2019-10-01T21:41:27.1097340Z 190 | / macro_rules! step_identical_methods {
2019-10-01T21:41:27.1097628Z 191 | |     () => {
2019-10-01T21:41:27.1097940Z 192 | |         #[inline]
2019-10-01T21:41:27.1098243Z 193 | |         fn successor(&self) -> Self {
2019-10-01T21:41:27.1098497Z ...   |
2019-10-01T21:41:27.1098805Z 204 | |             Self::unchecked_add(*self, 1)
2019-10-01T21:41:27.1099587Z     | |                   |
2019-10-01T21:41:27.1099912Z     | |                   function or associated item not found in `u8`
2019-10-01T21:41:27.1100248Z     | |                   help: there is a method with a similar name: `checked_add`
2019-10-01T21:41:27.1100521Z ...   |
2019-10-01T21:41:27.1100521Z ...   |
2019-10-01T21:41:27.1100792Z 221 | |     }
2019-10-01T21:41:27.1101075Z 222 | | }
2019-10-01T21:41:27.1101390Z     | |_- in this expansion of `step_identical_methods!`
2019-10-01T21:41:27.1101638Z 223 | 
2019-10-01T21:41:27.1101948Z 224 | / macro_rules! step_integer_impls {
2019-10-01T21:41:27.1103388Z 226 | |         narrower than or same width as usize:
2019-10-01T21:41:27.1103388Z 226 | |         narrower than or same width as usize:
2019-10-01T21:41:27.1103840Z 227 | |             $( [ $narrower_unsigned:ident $narrower_signed: ident ] ),+;
2019-10-01T21:41:27.1104097Z ...   |
2019-10-01T21:41:27.1104435Z 264 | |                 step_identical_methods!();
2019-10-01T21:41:27.1105750Z ...   |
2019-10-01T21:41:27.1106065Z 393 | |     }
2019-10-01T21:41:27.1106544Z 394 | | }
2019-10-01T21:41:27.1106544Z 394 | | }
2019-10-01T21:41:27.1106843Z     | |_- in this expansion of `step_integer_impls!`
2019-10-01T21:41:27.1107061Z ...
2019-10-01T21:41:27.1107344Z 397 | / step_integer_impls! {
2019-10-01T21:41:27.1107863Z 398 | |     narrower than or same width as usize: [u8 i8], [u16 i16], [u32 i32], [u64 i64], [usize isize];
2019-10-01T21:41:27.1108273Z 399 | |     wider than usize: [u128 i128];
2019-10-01T21:41:27.1109223Z     | |_- in this macro invocation
2019-10-01T21:41:27.1109286Z 
2019-10-01T21:41:27.1109286Z 
2019-10-01T21:41:27.1277442Z error[E0599]: no function or associated item named `unchecked_add` found for type `u16` in the current scope
2019-10-01T21:41:27.1277773Z    --> src/libcore/iter/range.rs:204:19
2019-10-01T21:41:27.1278027Z     |
2019-10-01T21:41:27.1278344Z 190 | / macro_rules! step_identical_methods {
2019-10-01T21:41:27.1278887Z 191 | |     () => {
2019-10-01T21:41:27.1279201Z 192 | |         #[inline]
2019-10-01T21:41:27.1279502Z 193 | |         fn successor(&self) -> Self {
2019-10-01T21:41:27.1279760Z ...   |
2019-10-01T21:41:27.1280063Z 204 | |             Self::unchecked_add(*self, 1)
2019-10-01T21:41:27.1280650Z     | |                   |
2019-10-01T21:41:27.1280984Z     | |                   function or associated item not found in `u16`
2019-10-01T21:41:27.1281319Z     | |                   help: there is a method with a similar name: `checked_add`
2019-10-01T21:41:27.1281590Z ...   |
2019-10-01T21:41:27.1281590Z ...   |
2019-10-01T21:41:27.1281858Z 221 | |     }
2019-10-01T21:41:27.1282138Z 222 | | }
2019-10-01T21:41:27.1283066Z     | |_- in this expansion of `step_identical_methods!`
2019-10-01T21:41:27.1283377Z 223 | 
2019-10-01T21:41:27.1283732Z 224 | / macro_rules! step_integer_impls {
2019-10-01T21:41:27.1284354Z 226 | |         narrower than or same width as usize:
2019-10-01T21:41:27.1284354Z 226 | |         narrower than or same width as usize:
2019-10-01T21:41:27.1284732Z 227 | |             $( [ $narrower_unsigned:ident $narrower_signed: ident ] ),+;
2019-10-01T21:41:27.1284985Z ...   |
2019-10-01T21:41:27.1285306Z 264 | |                 step_identical_methods!();
2019-10-01T21:41:27.1286371Z ...   |
2019-10-01T21:41:27.1286669Z 393 | |     }
2019-10-01T21:41:27.1286948Z 394 | | }
2019-10-01T21:41:27.1286948Z 394 | | }
2019-10-01T21:41:27.1287253Z     | |_- in this expansion of `step_integer_impls!`
2019-10-01T21:41:27.1287481Z ...
2019-10-01T21:41:27.1287876Z 397 | / step_integer_impls! {
2019-10-01T21:41:27.1288249Z 398 | |     narrower than or same width as usize: [u8 i8], [u16 i16], [u32 i32], [u64 i64], [usize isize];
2019-10-01T21:41:27.1288714Z 399 | |     wider than usize: [u128 i128];
2019-10-01T21:41:27.1289427Z     | |_- in this macro invocation
2019-10-01T21:41:27.1289469Z 
2019-10-01T21:41:27.1289469Z 
2019-10-01T21:41:27.1555617Z error[E0599]: no function or associated item named `unchecked_add` found for type `u32` in the current scope
2019-10-01T21:41:27.1555995Z    --> src/libcore/iter/range.rs:204:19
2019-10-01T21:41:27.1556362Z     |
2019-10-01T21:41:27.1556712Z 190 | / macro_rules! step_identical_methods {
2019-10-01T21:41:27.1557028Z 191 | |     () => {
2019-10-01T21:41:27.1557323Z 192 | |         #[inline]
2019-10-01T21:41:27.1557652Z 193 | |         fn successor(&self) -> Self {
2019-10-01T21:41:27.1557899Z ...   |
2019-10-01T21:41:27.1558210Z 204 | |             Self::unchecked_add(*self, 1)
2019-10-01T21:41:27.1558918Z     | |                   |
2019-10-01T21:41:27.1559247Z     | |                   function or associated item not found in `u32`
2019-10-01T21:41:27.1559608Z     | |                   help: there is a method with a similar name: `checked_add`
2019-10-01T21:41:27.1559847Z ...   |
2019-10-01T21:41:27.1559847Z ...   |
2019-10-01T21:41:27.1560132Z 221 | |     }
2019-10-01T21:41:27.1560401Z 222 | | }
2019-10-01T21:41:27.1560702Z     | |_- in this expansion of `step_identical_methods!`
2019-10-01T21:41:27.1560952Z 223 | 
2019-10-01T21:41:27.1561438Z 224 | / macro_rules! step_integer_impls {
2019-10-01T21:41:27.1562127Z 226 | |         narrower than or same width as usize:
2019-10-01T21:41:27.1562127Z 226 | |         narrower than or same width as usize:
2019-10-01T21:41:27.1562983Z 227 | |             $( [ $narrower_unsigned:ident $narrower_signed: ident ] ),+;
2019-10-01T21:41:27.1563554Z ...   |
2019-10-01T21:41:27.1563909Z 264 | |                 step_identical_methods!();
2019-10-01T21:41:27.1564800Z ...   |
2019-10-01T21:41:27.1565097Z 393 | |     }
2019-10-01T21:41:27.1565390Z 394 | | }
2019-10-01T21:41:27.1565390Z 394 | | }
2019-10-01T21:41:27.1565739Z     | |_- in this expansion of `step_integer_impls!`
2019-10-01T21:41:27.1565962Z ...
2019-10-01T21:41:27.1566403Z 397 | / step_integer_impls! {
2019-10-01T21:41:27.1566892Z 398 | |     narrower than or same width as usize: [u8 i8], [u16 i16], [u32 i32], [u64 i64], [usize isize];
2019-10-01T21:41:27.1567217Z 399 | |     wider than usize: [u128 i128];
2019-10-01T21:41:27.1567792Z     | |_- in this macro invocation
2019-10-01T21:41:27.1567847Z 
2019-10-01T21:41:27.1567847Z 
2019-10-01T21:41:27.1839009Z error[E0599]: no function or associated item named `unchecked_add` found for type `u64` in the current scope
2019-10-01T21:41:27.1839380Z    --> src/libcore/iter/range.rs:204:19
2019-10-01T21:41:27.1839656Z     |
2019-10-01T21:41:27.1839966Z 190 | / macro_rules! step_identical_methods {
2019-10-01T21:41:27.1840298Z 191 | |     () => {
2019-10-01T21:41:27.1840617Z 192 | |         #[inline]
2019-10-01T21:41:27.1840929Z 193 | |         fn successor(&self) -> Self {
2019-10-01T21:41:27.1841188Z ...   |
2019-10-01T21:41:27.1841502Z 204 | |             Self::unchecked_add(*self, 1)
2019-10-01T21:41:27.1842213Z     | |                   |
2019-10-01T21:41:27.1843355Z     | |                   function or associated item not found in `u64`
2019-10-01T21:41:27.1843828Z     | |                   help: there is a method with a similar name: `checked_add`
2019-10-01T21:41:27.1844116Z ...   |
2019-10-01T21:41:27.1844116Z ...   |
2019-10-01T21:41:27.1844405Z 221 | |     }
2019-10-01T21:41:27.1844704Z 222 | | }
2019-10-01T21:41:27.1845030Z     | |_- in this expansion of `step_identical_methods!`
2019-10-01T21:41:27.1845282Z 223 | 
2019-10-01T21:41:27.1845627Z 224 | / macro_rules! step_integer_impls {
2019-10-01T21:41:27.1846418Z 226 | |         narrower than or same width as usize:
2019-10-01T21:41:27.1846418Z 226 | |         narrower than or same width as usize:
2019-10-01T21:41:27.1846801Z 227 | |             $( [ $narrower_unsigned:ident $narrower_signed: ident ] ),+;
2019-10-01T21:41:27.1847057Z ...   |
2019-10-01T21:41:27.1847602Z 264 | |                 step_identical_methods!();
2019-10-01T21:41:27.1848234Z ...   |
2019-10-01T21:41:27.1848526Z 393 | |     }
2019-10-01T21:41:27.1848800Z 394 | | }
2019-10-01T21:41:27.1848800Z 394 | | }
2019-10-01T21:41:27.1849221Z     | |_- in this expansion of `step_integer_impls!`
2019-10-01T21:41:27.1849458Z ...
2019-10-01T21:41:27.1849750Z 397 | / step_integer_impls! {
2019-10-01T21:41:27.1850142Z 398 | |     narrower than or same width as usize: [u8 i8], [u16 i16], [u32 i32], [u64 i64], [usize isize];
2019-10-01T21:41:27.1850475Z 399 | |     wider than usize: [u128 i128];
2019-10-01T21:41:27.1851079Z     | |_- in this macro invocation
2019-10-01T21:41:27.1851120Z 
2019-10-01T21:41:27.1851120Z 
2019-10-01T21:41:27.1996647Z error[E0599]: no function or associated item named `unchecked_add` found for type `usize` in the current scope
2019-10-01T21:41:27.1997044Z    --> src/libcore/iter/range.rs:204:19
2019-10-01T21:41:27.1997287Z     |
2019-10-01T21:41:27.1997631Z 190 | / macro_rules! step_identical_methods {
2019-10-01T21:41:27.1998124Z 191 | |     () => {
2019-10-01T21:41:27.1998540Z 192 | |         #[inline]
2019-10-01T21:41:27.1998869Z 193 | |         fn successor(&self) -> Self {
2019-10-01T21:41:27.1999122Z ...   |
2019-10-01T21:41:27.1999461Z 204 | |             Self::unchecked_add(*self, 1)
2019-10-01T21:41:27.2000069Z     | |                   |
2019-10-01T21:41:27.2000439Z     | |                   function or associated item not found in `usize`
2019-10-01T21:41:27.2000951Z     | |                   help: there is a method with a similar name: `checked_add`
2019-10-01T21:41:27.2001210Z ...   |
2019-10-01T21:41:27.2001210Z ...   |
2019-10-01T21:41:27.2001521Z 221 | |     }
2019-10-01T21:41:27.2001807Z 222 | | }
2019-10-01T21:41:27.2002448Z     | |_- in this expansion of `step_identical_methods!`
2019-10-01T21:41:27.2002779Z 223 | 
2019-10-01T21:41:27.2003098Z 224 | / macro_rules! step_integer_impls {
2019-10-01T21:41:27.2003772Z 226 | |         narrower than or same width as usize:
2019-10-01T21:41:27.2003772Z 226 | |         narrower than or same width as usize:
2019-10-01T21:41:27.2004129Z 227 | |             $( [ $narrower_unsigned:ident $narrower_signed: ident ] ),+;
2019-10-01T21:41:27.2004405Z ...   |
2019-10-01T21:41:27.2004726Z 264 | |                 step_identical_methods!();
2019-10-01T21:41:27.2005384Z ...   |
2019-10-01T21:41:27.2005685Z 393 | |     }
2019-10-01T21:41:27.2005998Z 394 | | }
2019-10-01T21:41:27.2005998Z 394 | | }
2019-10-01T21:41:27.2006320Z     | |_- in this expansion of `step_integer_impls!`
2019-10-01T21:41:27.2006556Z ...
2019-10-01T21:41:27.2006863Z 397 | / step_integer_impls! {
2019-10-01T21:41:27.2007257Z 398 | |     narrower than or same width as usize: [u8 i8], [u16 i16], [u32 i32], [u64 i64], [usize isize];
2019-10-01T21:41:27.2007755Z 399 | |     wider than usize: [u128 i128];
2019-10-01T21:41:27.2008672Z     | |_- in this macro invocation
2019-10-01T21:41:27.2009202Z 
2019-10-01T21:41:27.2009202Z 
2019-10-01T21:41:27.2194342Z error[E0599]: no function or associated item named `unchecked_add` found for type `u128` in the current scope
2019-10-01T21:41:27.2195065Z    --> src/libcore/iter/range.rs:204:19
2019-10-01T21:41:27.2195579Z     |
2019-10-01T21:41:27.2196265Z 190 | / macro_rules! step_identical_methods {
2019-10-01T21:41:27.2196928Z 191 | |     () => {
2019-10-01T21:41:27.2197449Z 192 | |         #[inline]
2019-10-01T21:41:27.2198285Z 193 | |         fn successor(&self) -> Self {
2019-10-01T21:41:27.2198759Z ...   |
2019-10-01T21:41:27.2199409Z 204 | |             Self::unchecked_add(*self, 1)
2019-10-01T21:41:27.2200469Z     | |                   |
2019-10-01T21:41:27.2201042Z     | |                   function or associated item not found in `u128`
2019-10-01T21:41:27.2201631Z     | |                   help: there is a method with a similar name: `checked_add`
2019-10-01T21:41:27.2202329Z ...   |
2019-10-01T21:41:27.2202329Z ...   |
2019-10-01T21:41:27.2203490Z 221 | |     }
2019-10-01T21:41:27.2204065Z 222 | | }
2019-10-01T21:41:27.2204668Z     | |_- in this expansion of `step_identical_methods!`
2019-10-01T21:41:27.2205155Z 223 | 
2019-10-01T21:41:27.2205724Z 224 | / macro_rules! step_integer_impls {
2019-10-01T21:41:27.2206931Z 226 | |         narrower than or same width as usize:
2019-10-01T21:41:27.2206931Z 226 | |         narrower than or same width as usize:
2019-10-01T21:41:27.2207509Z 227 | |             $( [ $narrower_unsigned:ident $narrower_signed: ident ] ),+;
2019-10-01T21:41:27.2207967Z ...   |
2019-10-01T21:41:27.2208513Z 359 | |                 step_identical_methods!();
2019-10-01T21:41:27.2209712Z ...   |
2019-10-01T21:41:27.2210384Z 393 | |     }
2019-10-01T21:41:27.2211003Z 394 | | }
2019-10-01T21:41:27.2211003Z 394 | | }
2019-10-01T21:41:27.2211619Z     | |_- in this expansion of `step_integer_impls!`
2019-10-01T21:41:27.2212062Z ...
2019-10-01T21:41:27.2213204Z 397 | / step_integer_impls! {
2019-10-01T21:41:27.2213918Z 398 | |     narrower than or same width as usize: [u8 i8], [u16 i16], [u32 i32], [u64 i64], [usize isize];
2019-10-01T21:41:27.2214523Z 399 | |     wider than usize: [u128 i128];
2019-10-01T21:41:27.2215992Z     | |_- in this macro invocation
2019-10-01T21:41:27.2216262Z 
2019-10-01T21:41:27.2216262Z 
2019-10-01T21:41:27.2517295Z error[E0599]: no function or associated item named `unchecked_sub` found for type `u8` in the current scope
2019-10-01T21:41:27.2519249Z    --> src/libcore/iter/range.rs:219:19
2019-10-01T21:41:27.2520576Z     |
2019-10-01T21:41:27.2520945Z 190 | / macro_rules! step_identical_methods {
2019-10-01T21:41:27.2521253Z 191 | |     () => {
2019-10-01T21:41:27.2521585Z 192 | |         #[inline]
2019-10-01T21:41:27.2521953Z 193 | |         fn successor(&self) -> Self {
2019-10-01T21:41:27.2522209Z ...   |
2019-10-01T21:41:27.2522891Z 219 | |             Self::unchecked_sub(*self, 1)
2019-10-01T21:41:27.2523542Z     | |                   |
2019-10-01T21:41:27.2523897Z     | |                   function or associated item not found in `u8`
2019-10-01T21:41:27.2524273Z     | |                   help: there is a method with a similar name: `checked_sub`
2019-10-01T21:41:27.2524589Z 220 | |         }
2019-10-01T21:41:27.2524589Z 220 | |         }
2019-10-01T21:41:27.2524898Z 221 | |     }
2019-10-01T21:41:27.2525184Z 222 | | }
2019-10-01T21:41:27.2525509Z     | |_- in this expansion of `step_identical_methods!`
2019-10-01T21:41:27.2525779Z 223 | 
2019-10-01T21:41:27.2526225Z 224 | / macro_rules! step_integer_impls {
2019-10-01T21:41:27.2527296Z 226 | |         narrower than or same width as usize:
2019-10-01T21:41:27.2527296Z 226 | |         narrower than or same width as usize:
2019-10-01T21:41:27.2527726Z 227 | |             $( [ $narrower_unsigned:ident $narrower_signed: ident ] ),+;
2019-10-01T21:41:27.2528007Z ...   |
2019-10-01T21:41:27.2528334Z 264 | |                 step_identical_methods!();
2019-10-01T21:41:27.2528984Z ...   |
2019-10-01T21:41:27.2529283Z 393 | |     }
2019-10-01T21:41:27.2529761Z 394 | | }
2019-10-01T21:41:27.2529761Z 394 | | }
2019-10-01T21:41:27.2530092Z     | |_- in this expansion of `step_integer_impls!`
2019-10-01T21:41:27.2530314Z ...
2019-10-01T21:41:27.2530636Z 397 | / step_integer_impls! {
2019-10-01T21:41:27.2531023Z 398 | |     narrower than or same width as usize: [u8 i8], [u16 i16], [u32 i32], [u64 i64], [usize isize];
2019-10-01T21:41:27.2531369Z 399 | |     wider than usize: [u128 i128];
2019-10-01T21:41:27.2531993Z     | |_- in this macro invocation
2019-10-01T21:41:27.2532067Z 
2019-10-01T21:41:27.2532067Z 
2019-10-01T21:41:27.2836588Z error[E0599]: no function or associated item named `unchecked_sub` found for type `u16` in the current scope
2019-10-01T21:41:27.2836962Z    --> src/libcore/iter/range.rs:219:19
2019-10-01T21:41:27.2837207Z     |
2019-10-01T21:41:27.2837957Z 190 | / macro_rules! step_identical_methods {
2019-10-01T21:41:27.2838460Z 191 | |     () => {
2019-10-01T21:41:27.2838823Z 192 | |         #[inline]
2019-10-01T21:41:27.2839325Z 193 | |         fn successor(&self) -> Self {
2019-10-01T21:41:27.2839605Z ...   |
2019-10-01T21:41:27.2839928Z 219 | |             Self::unchecked_sub(*self, 1)
2019-10-01T21:41:27.2840548Z     | |                   |
2019-10-01T21:41:27.2840887Z     | |                   function or associated item not found in `u16`
2019-10-01T21:41:27.2841452Z     | |                   help: there is a method with a similar name: `checked_sub`
2019-10-01T21:41:27.2841963Z 220 | |         }
2019-10-01T21:41:27.2841963Z 220 | |         }
2019-10-01T21:41:27.2843001Z 221 | |     }
2019-10-01T21:41:27.2843327Z 222 | | }
2019-10-01T21:41:27.2843653Z     | |_- in this expansion of `step_identical_methods!`
2019-10-01T21:41:27.2843905Z 223 | 
2019-10-01T21:41:27.2844242Z 224 | / macro_rules! step_integer_impls {
2019-10-01T21:41:27.2845112Z 226 | |         narrower than or same width as usize:
2019-10-01T21:41:27.2845112Z 226 | |         narrower than or same width as usize:
2019-10-01T21:41:27.2845485Z 227 | |             $( [ $narrower_unsigned:ident $narrower_signed: ident ] ),+;
2019-10-01T21:41:27.2845863Z ...   |
2019-10-01T21:41:27.2846287Z 264 | |                 step_identical_methods!();
2019-10-01T21:41:27.2847078Z ...   |
2019-10-01T21:41:27.2847496Z 393 | |     }
2019-10-01T21:41:27.2847804Z 394 | | }
2019-10-01T21:41:27.2847804Z 394 | | }
2019-10-01T21:41:27.2848144Z     | |_- in this expansion of `step_integer_impls!`
2019-10-01T21:41:27.2848365Z ...
2019-10-01T21:41:27.2848669Z 397 | / step_integer_impls! {
2019-10-01T21:41:27.2849079Z 398 | |     narrower than or same width as usize: [u8 i8], [u16 i16], [u32 i32], [u64 i64], [usize isize];
2019-10-01T21:41:27.2849411Z 399 | |     wider than usize: [u128 i128];
2019-10-01T21:41:27.2850050Z     | |_- in this macro invocation
2019-10-01T21:41:27.2851101Z 
2019-10-01T21:41:27.2851101Z 
2019-10-01T21:41:27.2973511Z error[E0599]: no function or associated item named `unchecked_sub` found for type `u32` in the current scope
2019-10-01T21:41:27.2973907Z    --> src/libcore/iter/range.rs:219:19
2019-10-01T21:41:27.2974154Z     |
2019-10-01T21:41:27.2974476Z 190 | / macro_rules! step_identical_methods {
2019-10-01T21:41:27.2974794Z 191 | |     () => {
2019-10-01T21:41:27.2975102Z 192 | |         #[inline]
2019-10-01T21:41:27.2975637Z 193 | |         fn successor(&self) -> Self {
2019-10-01T21:41:27.2976014Z ...   |
2019-10-01T21:41:27.2976340Z 219 | |             Self::unchecked_sub(*self, 1)
2019-10-01T21:41:27.2977084Z     | |                   |
2019-10-01T21:41:27.2977412Z     | |                   function or associated item not found in `u32`
2019-10-01T21:41:27.2977906Z     | |                   help: there is a method with a similar name: `checked_sub`
2019-10-01T21:41:27.2978370Z 220 | |         }
2019-10-01T21:41:27.2978370Z 220 | |         }
2019-10-01T21:41:27.2978662Z 221 | |     }
2019-10-01T21:41:27.2978967Z 222 | | }
2019-10-01T21:41:27.2979295Z     | |_- in this expansion of `step_identical_methods!`
2019-10-01T21:41:27.2979589Z 223 | 
2019-10-01T21:41:27.2979954Z 224 | / macro_rules! step_integer_impls {
2019-10-01T21:41:27.2980648Z 226 | |         narrower than or same width as usize:
2019-10-01T21:41:27.2980648Z 226 | |         narrower than or same width as usize:
2019-10-01T21:41:27.2981070Z 227 | |             $( [ $narrower_unsigned:ident $narrower_signed: ident ] ),+;
2019-10-01T21:41:27.2981451Z ...   |
2019-10-01T21:41:27.2981779Z 264 | |                 step_identical_methods!();
2019-10-01T21:41:27.2982832Z ...   |
2019-10-01T21:41:27.2983286Z 393 | |     }
2019-10-01T21:41:27.2983609Z 394 | | }
2019-10-01T21:41:27.2983609Z 394 | | }
2019-10-01T21:41:27.2983959Z     | |_- in this expansion of `step_integer_impls!`
2019-10-01T21:41:27.2984184Z ...
2019-10-01T21:41:27.2984488Z 397 | / step_integer_impls! {
2019-10-01T21:41:27.2985173Z 398 | |     narrower than or same width as usize: [u8 i8], [u16 i16], [u32 i32], [u64 i64], [usize isize];
2019-10-01T21:41:27.2985562Z 399 | |     wider than usize: [u128 i128];
2019-10-01T21:41:27.2986357Z     | |_- in this macro invocation
2019-10-01T21:41:27.2986415Z 
2019-10-01T21:41:27.2986415Z 
2019-10-01T21:41:27.3104541Z error[E0599]: no function or associated item named `unchecked_sub` found for type `u64` in the current scope
2019-10-01T21:41:27.3105296Z    --> src/libcore/iter/range.rs:219:19
2019-10-01T21:41:27.3105604Z     |
2019-10-01T21:41:27.3105934Z 190 | / macro_rules! step_identical_methods {
2019-10-01T21:41:27.3106234Z 191 | |     () => {
2019-10-01T21:41:27.3106676Z 192 | |         #[inline]
2019-10-01T21:41:27.3107009Z 193 | |         fn successor(&self) -> Self {
2019-10-01T21:41:27.3107520Z ...   |
2019-10-01T21:41:27.3107856Z 219 | |             Self::unchecked_sub(*self, 1)
2019-10-01T21:41:27.3109705Z     | |                   |
2019-10-01T21:41:27.3110088Z     | |                   function or associated item not found in `u64`
2019-10-01T21:41:27.3110442Z     | |                   help: there is a method with a similar name: `checked_sub`
2019-10-01T21:41:27.3110792Z 220 | |         }
2019-10-01T21:41:27.3110792Z 220 | |         }
2019-10-01T21:41:27.3111089Z 221 | |     }
2019-10-01T21:41:27.3111374Z 222 | | }
2019-10-01T21:41:27.3112061Z     | |_- in this expansion of `step_identical_methods!`
2019-10-01T21:41:27.3112785Z 223 | 
2019-10-01T21:41:27.3113117Z 224 | / macro_rules! step_integer_impls {
2019-10-01T21:41:27.3113773Z 226 | |         narrower than or same width as usize:
2019-10-01T21:41:27.3113773Z 226 | |         narrower than or same width as usize:
2019-10-01T21:41:27.3114169Z 227 | |             $( [ $narrower_unsigned:ident $narrower_signed: ident ] ),+;
2019-10-01T21:41:27.3114426Z ...   |
2019-10-01T21:41:27.3114749Z 264 | |                 step_identical_methods!();
2019-10-01T21:41:27.3115405Z ...   |
2019-10-01T21:41:27.3115695Z 393 | |     }
2019-10-01T21:41:27.3116200Z 394 | | }
2019-10-01T21:41:27.3116200Z 394 | | }
2019-10-01T21:41:27.3116604Z     | |_- in this expansion of `step_integer_impls!`
2019-10-01T21:41:27.3116853Z ...
2019-10-01T21:41:27.3117154Z 397 | / step_integer_impls! {
2019-10-01T21:41:27.3117542Z 398 | |     narrower than or same width as usize: [u8 i8], [u16 i16], [u32 i32], [u64 i64], [usize isize];
2019-10-01T21:41:27.3117897Z 399 | |     wider than usize: [u128 i128];
2019-10-01T21:41:27.3118516Z     | |_- in this macro invocation
2019-10-01T21:41:27.3118673Z 
2019-10-01T21:41:27.3128304Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-10-01T21:41:27.3128304Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-10-01T21:41:27.3256391Z error[E0599]: no function or associated item named `unchecked_sub` found for type `usize` in the current scope
2019-10-01T21:41:27.3256733Z    --> src/libcore/iter/range.rs:219:19
2019-10-01T21:41:27.3256993Z     |
2019-10-01T21:41:27.3257307Z 190 | / macro_rules! step_identical_methods {
2019-10-01T21:41:27.3257721Z 191 | |     () => {
2019-10-01T21:41:27.3258065Z 192 | |         #[inline]
2019-10-01T21:41:27.3258408Z 193 | |         fn successor(&self) -> Self {
2019-10-01T21:41:27.3258662Z ...   |
2019-10-01T21:41:27.3259006Z 219 | |             Self::unchecked_sub(*self, 1)
2019-10-01T21:41:27.3259610Z     | |                   |
2019-10-01T21:41:27.3259971Z     | |                   function or associated item not found in `usize`
2019-10-01T21:41:27.3260334Z     | |                   help: there is a method with a similar name: `checked_sub`
2019-10-01T21:41:27.3260664Z 220 | |         }
2019-10-01T21:41:27.3260664Z 220 | |         }
2019-10-01T21:41:27.3260956Z 221 | |     }
2019-10-01T21:41:27.3261242Z 222 | | }
2019-10-01T21:41:27.3261585Z     | |_- in this expansion of `step_identical_methods!`
2019-10-01T21:41:27.3261836Z 223 | 
2019-10-01T21:41:27.3262154Z 224 | / macro_rules! step_integer_impls {
2019-10-01T21:41:27.3263364Z 226 | |         narrower than or same width as usize:
2019-10-01T21:41:27.3263364Z 226 | |         narrower than or same width as usize:
2019-10-01T21:41:27.3263821Z 227 | |             $( [ $narrower_unsigned:ident $narrower_signed: ident ] ),+;
2019-10-01T21:41:27.3264084Z ...   |
2019-10-01T21:41:27.3264409Z 264 | |                 step_identical_methods!();
2019-10-01T21:41:27.3265523Z ...   |
2019-10-01T21:41:27.3265832Z 393 | |     }
2019-10-01T21:41:27.3266522Z 394 | | }
2019-10-01T21:41:27.3266522Z 394 | | }
2019-10-01T21:41:27.3266837Z     | |_- in this expansion of `step_integer_impls!`
2019-10-01T21:41:27.3267074Z ...
2019-10-01T21:41:27.3267367Z 397 | / step_integer_impls! {
2019-10-01T21:41:27.3267880Z 398 | |     narrower than or same width as usize: [u8 i8], [u16 i16], [u32 i32], [u64 i64], [usize isize];
2019-10-01T21:41:27.3268232Z 399 | |     wider than usize: [u128 i128];
2019-10-01T21:41:27.3268869Z     | |_- in this macro invocation
2019-10-01T21:41:27.3268912Z 
2019-10-01T21:41:27.3268912Z 
2019-10-01T21:41:27.3410592Z error[E0599]: no function or associated item named `unchecked_sub` found for type `u128` in the current scope
2019-10-01T21:41:27.3411020Z    --> src/libcore/iter/range.rs:219:19
2019-10-01T21:41:27.3411266Z     |
2019-10-01T21:41:27.3411604Z 190 | / macro_rules! step_identical_methods {
2019-10-01T21:41:27.3411907Z 191 | |     () => {
2019-10-01T21:41:27.3412209Z 192 | |         #[inline]
2019-10-01T21:41:27.3413187Z 193 | |         fn successor(&self) -> Self {
2019-10-01T21:41:27.3413473Z ...   |
2019-10-01T21:41:27.3413797Z 219 | |             Self::unchecked_sub(*self, 1)
2019-10-01T21:41:27.3414421Z     | |                   |
2019-10-01T21:41:27.3414777Z     | |                   function or associated item not found in `u128`
2019-10-01T21:41:27.3415351Z     | |                   help: there is a method with a similar name: `checked_sub`
2019-10-01T21:41:27.3415739Z 220 | |         }
2019-10-01T21:41:27.3415739Z 220 | |         }
2019-10-01T21:41:27.3416051Z 221 | |     }
2019-10-01T21:41:27.3416336Z 222 | | }
2019-10-01T21:41:27.3416659Z     | |_- in this expansion of `step_identical_methods!`
2019-10-01T21:41:27.3416929Z 223 | 
2019-10-01T21:41:27.3417243Z 224 | / macro_rules! step_integer_impls {
2019-10-01T21:41:27.3418110Z 226 | |         narrower than or same width as usize:
2019-10-01T21:41:27.3418110Z 226 | |         narrower than or same width as usize:
2019-10-01T21:41:27.3418480Z 227 | |             $( [ $narrower_unsigned:ident $narrower_signed: ident ] ),+;
2019-10-01T21:41:27.3418751Z ...   |
2019-10-01T21:41:27.3419131Z 359 | |                 step_identical_methods!();
2019-10-01T21:41:27.3419795Z ...   |
2019-10-01T21:41:27.3420093Z 393 | |     }
2019-10-01T21:41:27.3420417Z 394 | | }
2019-10-01T21:41:27.3420417Z 394 | | }
2019-10-01T21:41:27.3420741Z     | |_- in this expansion of `step_integer_impls!`
2019-10-01T21:41:27.3420961Z ...
2019-10-01T21:41:27.3421283Z 397 | / step_integer_impls! {
2019-10-01T21:41:27.3421675Z 398 | |     narrower than or same width as usize: [u8 i8], [u16 i16], [u32 i32], [u64 i64], [usize isize];
2019-10-01T21:41:27.3422025Z 399 | |     wider than usize: [u128 i128];
2019-10-01T21:41:27.3423296Z     | |_- in this macro invocation
2019-10-01T21:41:27.3423361Z 
2019-10-01T21:41:28.9868406Z    Compiling libc v0.2.62
2019-10-01T21:41:29.9282456Z    Compiling std v0.0.0 (/checkout/src/libstd)
---
2019-10-01T21:41:32.4887651Z == clock drift check ==
2019-10-01T21:41:32.4911882Z   local time: Tue Oct  1 21:41:32 UTC 2019
2019-10-01T21:41:32.6406139Z   network time: Tue, 01 Oct 2019 21:41:32 GMT
2019-10-01T21:41:32.6409956Z == end clock drift check ==
2019-10-01T21:41:45.9578531Z ##[error]Bash exited with code '1'.
2019-10-01T21:41:45.9647057Z ##[section]Starting: Checkout
2019-10-01T21:41:45.9649174Z ==============================================================================
2019-10-01T21:41:45.9649227Z Task         : Get sources
2019-10-01T21:41:45.9649269Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
