plain
[00:20:02]    Compiling unwind v0.0.0 (/checkout/src/libunwind)
[00:20:09]    Compiling compiler_builtins v0.0.0 (/checkout/src/rustc/compiler_builtins_shim)
[00:20:09]    Compiling cmake v0.1.33
[00:20:09]    Compiling alloc_jemalloc v0.0.0 (/checkout/src/liballoc_jemalloc)
[00:20:10] error: unused import: `prelude::v1::*`
[00:20:10]     |
[00:20:10]     |
[00:20:10] 135 | use prelude::v1::*;
[00:20:10]     |
[00:20:10]     = note: `-D unused-imports` implied by `-D warnings`
[00:20:10] 
[00:20:10] error: unused macro definition
[00:20:10] error: unused macro definition
[00:20:10]   --> libcore/num/wrapping.rs:16:1
[00:20:10]    |
[00:20:10] 16 | / macro_rules! sh_impl_signed {
[00:20:10] 17 | |     ($t:ident, $f:ident) => (
[00:20:10] 18 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:20:10] 19 | |         impl Shl<$f> for Wrapping<$t> {
[00:20:10] 63 | |     )
[00:20:10] 64 | | }
[00:20:10]    | |_^
[00:20:10]    |
[00:20:10]    |
[00:20:10]    = note: `-D unused-macros` implied by `-D warnings`
[00:20:10] 
[00:20:10] error: unused macro definition
[00:20:10]    --> libcore/ops/arith.rs:636:1
[00:20:10]     |
[00:20:10] 636 | / macro_rules! neg_impl_unsigned {
[00:20:10] 637 | |     ($($t:ty)*) => {
[00:20:10] 638 | |         neg_impl_core!{ x => {
[00:20:10] 639 | |             !x.wrapping_add(1)
[00:20:10] 640 | |         }, $($t)*} }
[00:20:10]     | |_^
[00:20:10] 
[00:20:10] error: unused macro definition
[00:20:10]    --> libcore/lib.rs:237:1
[00:20:10]    --> libcore/lib.rs:237:1
[00:20:10]     |
[00:20:10] 237 | macro_rules! test_v16 { ($item:item) => {}; }
[00:20:10] 
[00:20:10] error: unused macro definition
[00:20:10]    --> libcore/lib.rs:239:1
[00:20:10]     |
[00:20:10]     |
[00:20:10] 239 | macro_rules! test_v32 { ($item:item) => {}; }
[00:20:10] 
[00:20:10] error: unused macro definition
[00:20:10]    --> libcore/lib.rs:241:1
[00:20:10]     |
[00:20:10]     |
[00:20:10] 241 | macro_rules! test_v64 { ($item:item) => {}; }
[00:20:10] 
[00:20:10] error: unused macro definition
[00:20:10]    --> libcore/lib.rs:243:1
[00:20:10]     |
[00:20:10]     |
[00:20:10] 243 | macro_rules! test_v128 { ($item:item) => {}; }
[00:20:10] 
[00:20:10] error: unused macro definition
[00:20:10]    --> libcore/lib.rs:245:1
[00:20:10]     |
[00:20:10]     |
[00:20:10] 245 | macro_rules! test_v256 { ($item:item) => {}; }
[00:20:10] 
[00:20:10] error: unused macro definition
[00:20:10]    --> libcore/lib.rs:247:1
[00:20:10]     |
[00:20:10]     |
[00:20:10] 247 | macro_rules! test_v512 { ($item:item) => {}; }
[00:20:10] 
[00:20:10] error: unused macro definition
[00:20:10]    --> libcore/lib.rs:249:1
[00:20:10]     |
[00:20:10]     |
[00:20:10] 249 | macro_rules! vector_impl { ($([$f:ident, $($args:tt)*]),*) => { $($f!($($args)*);)* } }
[00:20:10] 
[00:20:10] 
[00:20:12] error: use of deprecated item 'str::LinesAny': use lines()/Lines instead now
[00:20:12]      |
[00:20:12]      |
[00:20:12] 1387 | impl<'a> Iterator for LinesAny<'a> {
[00:20:12]      |
[00:20:12]      = note: `-D deprecated` implied by `-D warnings`
[00:20:12] 
[00:20:12] 
[00:20:12] error: use of deprecated item 'str::LinesAny': use lines()/Lines instead now
[00:20:12]      |
[00:20:12]      |
[00:20:12] 1403 | impl<'a> DoubleEndedIterator for LinesAny<'a> {
[00:20:12] 
[00:20:12] 
[00:20:12] error: use of deprecated item 'str::LinesAny': use lines()/Lines instead now
[00:20:12]      |
[00:20:12]      |
[00:20:12] 1412 | impl FusedIterator for LinesAny<'_> {}
[00:20:12] 
[00:20:12] 
[00:20:12] error: use of deprecated item 'hash::sip::SipHasher': use `std::collections::hash_map::DefaultHasher` instead
[00:20:12]    --> libcore/hash/mod.rs:100:9
[00:20:12]     |
[00:20:12] 100 | pub use self::sip::SipHasher;
[00:20:12] 
[00:20:12] 
[00:20:12] error: use of deprecated item 'hash::sip::SipHasher13': use `std::collections::hash_map::DefaultHasher` instead
[00:20:12]    --> libcore/hash/mod.rs:105:9
[00:20:12]     |
[00:20:12] 105 | pub use self::sip::SipHasher13;
[00:20:12] 
[00:20:12] 
[00:20:12] error: use of deprecated item 'hash::sip::SipHasher': use `std::collections::hash_map::DefaultHasher` instead
[00:20:12]    --> libcore/hash/sip.rs:144:6
[00:20:12] 144 | impl SipHasher {
[00:20:12]     |      ^^^^^^^^^
[00:20:12] 
[00:20:12] 
[00:20:12] error: use of deprecated item 'hash::sip::SipHasher13': use `std::collections::hash_map::DefaultHasher` instead
[00:20:12]    --> libcore/hash/sip.rs:166:6
[00:20:12] 166 | impl SipHasher13 {
[00:20:12]     |      ^^^^^^^^^^^
[00:20:12] 
[00:20:12] 
[00:20:12] error: use of deprecated item 'hash::sip::SipHasher': use `std::collections::hash_map::DefaultHasher` instead
[00:20:12]    --> libcore/hash/sip.rs:252:24
[00:20:12]     |
[00:20:12] 252 | impl super::Hasher for SipHasher {
[00:20:12] 
[00:20:12] 
[00:20:12] error: use of deprecated item 'hash::sip::SipHasher13': use `std::collections::hash_map::DefaultHasher` instead
[00:20:12]    --> libcore/hash/sip.rs:265:24
[00:20:12]     |
[00:20:12] 265 | impl super::Hasher for SipHasher13 {
[00:20:12] 
[00:20:12] 
[00:20:12] error: use of deprecated item 'hash::sip::SipHasher24': use `std::collections::hash_map::DefaultHasher` instead
[00:20:12]   --> libcore/hash/sip.rs:62:22
[00:20:12]    |
[00:20:12] 62 | pub struct SipHasher(SipHasher24);
[00:20:12] 
[00:20:12] 
[00:20:12] error: use of deprecated item 'str::LinesAny': use lines()/Lines instead now
[00:20:12]      |
[00:20:12]      |
[00:20:12] 2852 |     pub fn lines_any(&self) -> LinesAny {
[00:20:12] 
[00:20:12] 
[00:20:12] error: use of deprecated item 'str::LinesAny': use lines()/Lines instead now
[00:20:12]      |
[00:20:12]      |
[00:20:12] 2853 |         LinesAny(self.lines())
[00:20:12] 
[00:20:12] 
[00:20:12] error: use of deprecated item 'hash::sip::SipHasher': use `std::collections::hash_map::DefaultHasher` instead
[00:20:12]    --> libcore/hash/sip.rs:150:21
[00:20:12]     |
[00:20:12] 150 |     pub fn new() -> SipHasher {
[00:20:12] 
[00:20:12] 
[00:20:12] error: use of deprecated item 'hash::sip::SipHasher': use `std::collections::hash_map::DefaultHasher` instead
[00:20:12]    --> libcore/hash/sip.rs:151:9
[00:20:12]     |
[00:20:12] 151 |         SipHasher::new_with_keys(0, 0)
[00:20:12] 
[00:20:12] 
[00:20:12] error: use of deprecated item 'hash::sip::SipHasher': use `std::collections::hash_map::DefaultHasher` instead
[00:20:12]    --> libcore/hash/sip.rs:159:51
[00:20:12]     |
[00:20:12] 159 |     pub fn new_with_keys(key0: u64, key1: u64) -> SipHasher {
[00:20:12] 
[00:20:12] 
[00:20:12] error: use of deprecated item 'hash::sip::SipHasher': use `std::collections::hash_map::DefaultHasher` instead
[00:20:12]    --> libcore/hash/sip.rs:160:9
[00:20:12]     |
[00:20:12] 160 |         SipHasher(SipHasher24 {
[00:20:12] 
[00:20:12] 
[00:20:12] error: use of deprecated item 'hash::sip::SipHasher24': use `std::collections::hash_map::DefaultHasher` instead
[00:20:12]    --> libcore/hash/sip.rs:160:19
[00:20:12]     |
[00:20:12] 160 |         SipHasher(SipHasher24 {
[00:20:12] 
[00:20:12] 
[00:20:12] error: use of deprecated item 'hash::sip::SipHasher13': use `std::collections::hash_map::DefaultHasher` instead
[00:20:12]    --> libcore/hash/sip.rs:172:21
[00:20:12]     |
[00:20:12] 172 |     pub fn new() -> SipHasher13 {
[00:20:12] 
[00:20:12] 
[00:20:12] error: use of deprecated item 'hash::sip::SipHasher13': use `std::collections::hash_map::DefaultHasher` instead
[00:20:12]    --> libcore/hash/sip.rs:173:9
[00:20:12]     |
[00:20:12] 173 |         SipHasher13::new_with_keys(0, 0)
[00:20:12] 
[00:20:12] 
[00:20:12] error: use of deprecated item 'hash::sip::SipHasher13': use `std::collections::hash_map::DefaultHasher` instead
[00:20:12]    --> libcore/hash/sip.rs:181:51
[00:20:12]     |
[00:20:12] 181 |     pub fn new_with_keys(key0: u64, key1: u64) -> SipHasher13 {
[00:20:12] 
[00:20:12] 
[00:20:12] error: use of deprecated item 'hash::sip::SipHasher13': use `std::collections::hash_map::DefaultHasher` instead
[00:20:12]    --> libcore/hash/sip.rs:182:9
[00:20:12] 182 |         SipHasher13 {
[00:20:12]     |         ^^^^^^^^^^^
[00:20:12] 
[00:20:12]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:20:12]    Compiling std v0.0.0 (/checkout/src/libstd)
[00:20:12] error: use of deprecated item 'str::LinesAny': use lines()/Lines instead now
[00:20:12] 
[00:20:12] error: use of deprecated item 'hash::sip::SipHasher13': use `std::collections::hash_map::DefaultHasher` instead
[00:20:12] 
[00:20:12] error: use of deprecated item 'hash::sip::SipHasher24': use `std::collections::hash_map::DefaultHasher` instead
[00:20:12] 
[00:20:12] error: use of deprecated item 'hash::sip::SipHasher': use `std::collections::hash_map::DefaultHasher` instead
[00:20:14]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:20:14]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:20:14]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:20:15]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:20:15]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:20:25] error: use of deprecated item 'hash::sip::SipHasher::new_with_keys': use `std::collections::hash_map::DefaultHasher` instead
[00:20:25]    --> libcore/hash/sip.rs:151:9
[00:20:25]     |
[00:20:25] 151 |         SipHasher::new_with_keys(0, 0)
[00:20:25] 
[00:20:25] 
[00:20:25] error: use of deprecated item 'hash::sip::SipHasher24::hasher': use `std::collections::hash_map::DefaultHasher` instead
[00:20:25]    --> libcore/hash/sip.rs:161:13
[00:20:25]     |
[00:20:25] 161 |             hasher: Hasher::new_with_keys(key0, key1)
[00:20:25] 
[00:20:25] 
[00:20:25] error: use of deprecated item 'hash::sip::SipHasher13::new_with_keys': use `std::collections::hash_map::DefaultHasher` instead
[00:20:25]    --> libcore/hash/sip.rs:173:9
[00:20:25]     |
[00:20:25] 173 |         SipHasher13::new_with_keys(0, 0)
[00:20:25] 
[00:20:25] 
[00:20:25] error: use of deprecated item 'hash::sip::SipHasher13::hasher': use `std::collections::hash_map::DefaultHasher` instead
[00:20:25]    --> libcore/hash/sip.rs:183:13
[00:20:25]     |
[00:20:25] 183 |             hasher: Hasher::new_with_keys(key0, key1)
[00:20:25] 
[00:20:25] 
[00:20:25] error: use of deprecated item 'hash::sip::SipHasher24::hasher': use `std::collections::hash_map::DefaultHasher` instead
[00:20:25]    --> libcore/hash/sip.rs:255:9
[00:20:25]     |
[00:20:25] 255 |         self.0.hasher.write(msg)
[00:20:25] 
[00:20:25] 
[00:20:25] error: use of deprecated item 'hash::sip::SipHasher24::hasher': use `std::collections::hash_map::DefaultHasher` instead
[00:20:25]    --> libcore/hash/sip.rs:260:9
[00:20:25]     |
[00:20:25] 260 |         self.0.hasher.finish()
[00:20:25] 
[00:20:25] 
[00:20:25] error: use of deprecated item 'hash::sip::SipHasher13::hasher': use `std::collections::hash_map::DefaultHasher` instead
[00:20:25]    --> libcore/hash/sip.rs:268:9
[00:20:25]     |
[00:20:25] 268 |         self.hasher.write(msg)
[00:20:25] 
[00:20:25] 
[00:20:25] error: use of deprecated item 'hash::sip::SipHasher13::hasher': use `std::collections::hash_map::DefaultHasher` instead
[00:20:25]    --> libcore/hash/sip.rs:273:9
[00:20:25] 273 |         self.hasher.finish()
[00:20:25]     |         ^^^^^^^^^^^
[00:20:25] 
/modules/compiler-rt/objects
---
travis_time:end:09fe1344:start=1539331646700284145,finish=1539331646705078475,duration=4794330
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0668a818
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09e83770
travis_time:start:09e83770
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-g
