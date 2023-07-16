plain
travis_time:end:0d3485ee:start=1554061409017553506,finish=1554061411561916353,duration=2544362847
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:04:57]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:05:03] error: method is never used: `value`
[00:05:03]   --> src/libstd/sys_common/wtf8.rs:47:5
[00:05:03]    |
[00:05:03] 47 |     pub(super) fn value(self) -> u16 {
[00:05:03]    |
[00:05:03]    = note: `-D dead-code` implied by `-D warnings`
[00:05:03] 
[00:05:03] error: method is never used: `value`
[00:05:03] error: method is never used: `value`
[00:05:03]   --> src/libstd/sys_common/wtf8.rs:66:5
[00:05:03]    |
[00:05:03] 66 |     pub(super) fn value(self) -> u16 {
[00:05:03] 
[00:05:03] 
[00:05:03] error: function is never used: `decode_surrogate_pair`
[00:05:03]    |
[00:05:03]    |
[00:05:03] 71 | fn decode_surrogate_pair(high: HighSurrogate, low: LowSurrogate) -> [u8; 4] {
[00:05:03] 
[00:05:03] error: method is never used: `with_capacity`
[00:05:03]    --> src/libstd/sys_common/wtf8.rs:236:5
[00:05:03]     |
[00:05:03]     |
[00:05:03] 236 |     pub fn with_capacity(n: usize) -> Self {
[00:05:03] 
[00:05:03] error: method is never used: `from_string`
[00:05:03]    --> src/libstd/sys_common/wtf8.rs:246:5
[00:05:03]     |
[00:05:03]     |
[00:05:03] 246 |     pub fn from_string(string: String) -> Self {
[00:05:03]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:05:03] 
[00:05:03] error: method is never used: `clear`
[00:05:03]    --> src/libstd/sys_common/wtf8.rs:262:5
[00:05:03]     |
[00:05:03] 262 |     pub fn clear(&mut self) {
[00:05:03] 
[00:05:03] error: method is never used: `from_wide`
[00:05:03]    --> src/libstd/sys_common/wtf8.rs:270:5
[00:05:03]     |
[00:05:03]     |
[00:05:03] 270 |     pub fn from_wide(ucs2: &[u16]) -> Self {
[00:05:03] 
[00:05:03] error: function is never used: `encode_unit`
[00:05:03]    --> src/libstd/sys_common/wtf8.rs:271:9
[00:05:03]     |
[00:05:03]     |
[00:05:03] 271 |         fn encode_unit(buf: &mut Vec<u8>, c: u16) {
[00:05:03] 
[00:05:03] error: method is never used: `reserve`
[00:05:03]    --> src/libstd/sys_common/wtf8.rs:339:5
[00:05:03]     |
[00:05:03]     |
[00:05:03] 339 |     pub fn reserve(&mut self, additional: usize) {
[00:05:03] 
[00:05:03] error: method is never used: `reserve_exact`
[00:05:03]    --> src/libstd/sys_common/wtf8.rs:344:5
[00:05:03]     |
[00:05:03]     |
[00:05:03] 344 |     pub fn reserve_exact(&mut self, additional: usize) {
[00:05:03] 
[00:05:03] error: method is never used: `shrink_to_fit`
[00:05:03]    --> src/libstd/sys_common/wtf8.rs:349:5
[00:05:03]     |
[00:05:03]     |
[00:05:03] 349 |     pub fn shrink_to_fit(&mut self) {
[00:05:03] 
[00:05:03] error: method is never used: `shrink_to`
[00:05:03]    --> src/libstd/sys_common/wtf8.rs:354:5
[00:05:03]     |
[00:05:03]     |
[00:05:03] 354 |     pub fn shrink_to(&mut self, min_capacity: usize) {
[00:05:03] 
[00:05:03] error: method is never used: `capacity`
[00:05:03]    --> src/libstd/sys_common/wtf8.rs:360:5
[00:05:03]     |
[00:05:03]     |
[00:05:03] 360 |     pub fn capacity(&self) -> usize {
[00:05:03]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:05:03] 
[00:05:03] error: method is never used: `push_wtf8`
[00:05:03]    --> src/libstd/sys_common/wtf8.rs:377:5
[00:05:03]     |
[00:05:03] 377 |     pub fn push_wtf8(&mut self, other: &Wtf8) {
[00:05:03] 
[00:05:03] error: method is never used: `make_ascii_uppercase`
[00:05:03]    --> src/libstd/sys_common/wtf8.rs:413:5
[00:05:03]     |
---
[00:05:03] 
[00:05:03] error: method is never used: `into_box`
[00:05:03]    --> src/libstd/sys_common/wtf8.rs:453:5
[00:05:03]     |
[00:05:03] 453 |     pub fn into_box(self) -> Box<Wtf8> {
[00:05:03] 
[00:05:03] error: method is never used: `from_box`
[00:05:03]    --> src/libstd/sys_common/wtf8.rs:458:5
[00:05:03]     |
[00:05:03]     |
[00:05:03] 458 |     pub fn from_box(boxed: Box<Wtf8>) -> Self {
[00:05:03] 
[00:05:03] error: method is never used: `is_empty`
[00:05:03]    --> src/libstd/sys_common/wtf8.rs:562:5
[00:05:03]     |
[00:05:03]     |
[00:05:03] 562 |     pub fn is_empty(&self) -> bool {
[00:05:03]     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:05:03] 
[00:05:03] error: method is never used: `as_str`
[00:05:03]    --> src/libstd/sys_common/wtf8.rs:587:5
[00:05:03]     |
[00:05:03] 587 |     pub fn as_str(&self) -> Option<&str> {
[00:05:03] 
[00:05:03] error: method is never used: `to_string_lossy`
[00:05:03]    --> src/libstd/sys_common/wtf8.rs:602:5
[00:05:03]     |
[00:05:03]     |
[00:05:03] 602 |     pub fn to_string_lossy(&self) -> Cow<'_, str> {
[00:05:03] 
[00:05:03] error: method is never used: `encode_wide`
[00:05:03]    --> src/libstd/sys_common/wtf8.rs:634:5
[00:05:03]     |
[00:05:03]     |
[00:05:03] 634 |     pub fn encode_wide(&self) -> EncodeWide<'_> {
[00:05:03] 
[00:05:03] error: method is never used: `canonicalize_in_place`
[00:05:03]    --> src/libstd/sys_common/wtf8.rs:680:5
[00:05:03]     |
[00:05:03]     |
[00:05:03] 680 |     fn canonicalize_in_place(bytes: &mut [u8]) {
[00:05:03] 
[00:05:03] 
[00:05:03] error: method is never used: `empty_box`
[00:05:03]     |
[00:05:03]     |
[00:05:03] 703 |     pub fn empty_box() -> Box<Wtf8> {
[00:05:03] 
[00:05:03] error: method is never used: `into_box`
[00:05:03]    --> src/libstd/sys_common/wtf8.rs:709:5
[00:05:03]     |
[00:05:03]     |
[00:05:03] 709 |     pub fn into_box(&self) -> Box<Wtf8> {
[00:05:03] 
[00:05:03] error: method is never used: `into_arc`
[00:05:03]    --> src/libstd/sys_common/wtf8.rs:716:5
[00:05:03]     |
[00:05:03]     |
[00:05:03] 716 |     pub fn into_arc(&self) -> Arc<Wtf8> {
[00:05:03] 
[00:05:03] error: method is never used: `into_rc`
[00:05:03]    --> src/libstd/sys_common/wtf8.rs:727:5
[00:05:03]     |
[00:05:03]     |
[00:05:03] 727 |     pub fn into_rc(&self) -> Rc<Wtf8> {
[00:05:03] 
[00:05:03] error: method is never used: `new`
[00:05:03]     --> src/libstd/sys_common/wtf8.rs:1532:5
[00:05:03]      |
[00:05:03]      |
[00:05:03] 1532 |     fn new(ls: LowSurrogate) -> Self {
[00:05:03] 
[00:05:03] error: method is never used: `new`
[00:05:03]     --> src/libstd/sys_common/wtf8.rs:1560:5
[00:05:03]      |
[00:05:03]      |
[00:05:03] 1560 |     fn new(hs: HighSurrogate) -> Self {
[00:05:03] 
[00:05:03] 
[00:05:03] error: function is never used: `new_wtf8_searcher`
[00:05:03]      |
[00:05:03]      |
[00:05:03] 1611 | pub fn new_wtf8_searcher(needle: &Wtf8) -> Wtf8Searcher<SliceSearcher<'_, u8>> {
[00:05:03] 
[00:05:03] 
[00:05:03] error: function is never used: `new_wtf8_consumer`
[00:05:03]      |
[00:05:03]      |
[00:05:03] 1620 | pub fn new_wtf8_consumer(needle: &Wtf8) -> Wtf8Searcher<NaiveSearcher<'_, u8>> {
[00:05:03] 
[00:05:03] error: aborting due to 31 previous errors
[00:05:03] 
[00:05:03] error: Could not compile `std`.
---
travis_time:end:08b544c6:start=1554061726398872918,finish=1554061726404392970,duration=5520052
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1380fb64
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0070e35e
travis_time:start:0070e35e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_fail
