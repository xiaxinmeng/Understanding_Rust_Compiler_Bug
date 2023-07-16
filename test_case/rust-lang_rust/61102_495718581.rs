plain
travis_time:end:0b4c7c0c:start=1558716778630855058,finish=1558716868372606532,duration=89741751474
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:27:38]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:27:53] error[E0382]: use of moved value
[00:27:53]    --> src/libcore/num/flt2dec/mod.rs:193:25
[00:27:53]     |
[00:27:53] 193 |                     for c in &mut out[..nzeroes] { *c = b'0'; }
[00:27:53]     |                         ^ value moved here, in previous iteration of loop
[00:27:53]     |
[00:27:53]     = note: move occurs because value has type `&mut u8`, which does not implement the `Copy` trait
[00:27:53] error[E0382]: use of moved value
[00:27:53]    --> src/libcore/num/flt2dec/mod.rs:196:25
[00:27:53]     |
[00:27:53]     |
[00:27:53] 196 |                     for c in out[..len].iter_mut().rev() {
[00:27:53]     |                         ^ value moved here, in previous iteration of loop
[00:27:53]     |
[00:27:53]     = note: move occurs because value has type `&mut u8`, which does not implement the `Copy` trait
[00:27:53] error[E0382]: use of moved value
[00:27:53]    --> src/libcore/num/flt2dec/strategy/dragon.rs:279:21
[00:27:53]     |
[00:27:53]     |
[00:27:53] 279 |                 for c in &mut buf[i..len] { *c = b'0'; }
[00:27:53]     |                     ^ value moved here, in previous iteration of loop
[00:27:53]     |
[00:27:53]     = note: move occurs because value has type `&mut u8`, which does not implement the `Copy` trait
[00:27:53] error[E0382]: use of moved value
[00:27:53]    --> src/libcore/num/bignum.rs:253:21
[00:27:53]     |
[00:27:53] 100 | / macro_rules! define_bignum {
[00:27:53] 100 | / macro_rules! define_bignum {
[00:27:53] 101 | |     ($name:ident: type=$ty:ty, n=$n:expr) => (
[00:27:53] 102 | |         /// Stack-allocated arbitrary-precision (up to certain limit) integer.
[00:27:53] ...   |
[00:27:53] ...   |
[00:27:53] 253 | |                 for a in &mut self.base[..sz] {
[00:27:53]     | |                     ^ value moved here, in previous iteration of loop
[00:27:53] 475 | |     )
[00:27:53] 476 | | }
[00:27:53]     | |_- in this expansion of `define_bignum!`
[00:27:53] ...
[00:27:53] ...
[00:27:53] 481 |   define_bignum!(Big32x40: type=Digit32, n=40);
[00:27:53]     |
[00:27:53]     |
[00:27:53]     = note: move occurs because value has type `&mut u32`, which does not implement the `Copy` trait
[00:27:53] error[E0382]: use of moved value
[00:27:53]    --> src/libcore/num/bignum.rs:253:21
[00:27:53]     |
[00:27:53] 100 | / macro_rules! define_bignum {
[00:27:53] 100 | / macro_rules! define_bignum {
[00:27:53] 101 | |     ($name:ident: type=$ty:ty, n=$n:expr) => (
[00:27:53] 102 | |         /// Stack-allocated arbitrary-precision (up to certain limit) integer.
[00:27:53] ...   |
[00:27:53] ...   |
[00:27:53] 253 | |                 for a in &mut self.base[..sz] {
[00:27:53]     | |                     ^ value moved here, in previous iteration of loop
[00:27:53] 475 | |     )
[00:27:53] 476 | | }
[00:27:53]     | |_- in this expansion of `define_bignum!`
[00:27:53] ...
[00:27:53] ...
[00:27:53] 486 |       define_bignum!(Big8x3: type=u8, n=3);
[00:27:53]     |
[00:27:53]     |
[00:27:53]     = note: move occurs because value has type `&mut u8`, which does not implement the `Copy` trait
[00:27:53] error[E0382]: use of moved value
[00:27:53]    --> src/libcore/num/bignum.rs:384:21
[00:27:53]     |
[00:27:53] 100 | / macro_rules! define_bignum {
[00:27:53] 100 | / macro_rules! define_bignum {
[00:27:53] 101 | |     ($name:ident: type=$ty:ty, n=$n:expr) => (
[00:27:53] 102 | |         /// Stack-allocated arbitrary-precision (up to certain limit) integer.
[00:27:53] ...   |
[00:27:53] ...   |
[00:27:53] 384 | |                 for a in self.base[..sz].iter_mut().rev() {
[00:27:53]     | |                     ^ value moved here, in previous iteration of loop
[00:27:53] 475 | |     )
[00:27:53] 476 | | }
[00:27:53]     | |_- in this expansion of `define_bignum!`
[00:27:53] ...
[00:27:53] ...
[00:27:53] 481 |   define_bignum!(Big32x40: type=Digit32, n=40);
[00:27:53]     |
[00:27:53]     |
[00:27:53]     = note: move occurs because value has type `&mut u32`, which does not implement the `Copy` trait
[00:27:53] error[E0382]: use of moved value
[00:27:53]    --> src/libcore/num/bignum.rs:384:21
[00:27:53]     |
[00:27:53] 100 | / macro_rules! define_bignum {
[00:27:53] 100 | / macro_rules! define_bignum {
[00:27:53] 101 | |     ($name:ident: type=$ty:ty, n=$n:expr) => (
[00:27:53] 102 | |         /// Stack-allocated arbitrary-precision (up to certain limit) integer.
[00:27:53] ...   |
[00:27:53] ...   |
[00:27:53] 384 | |                 for a in self.base[..sz].iter_mut().rev() {
[00:27:53]     | |                     ^ value moved here, in previous iteration of loop
[00:27:53] 475 | |     )
[00:27:53] 476 | | }
[00:27:53]     | |_- in this expansion of `define_bignum!`
[00:27:53] ...
[00:27:53] ...
[00:27:53] 486 |       define_bignum!(Big8x3: type=u8, n=3);
[00:27:53]     |
[00:27:53]     |
[00:27:53]     = note: move occurs because value has type `&mut u8`, which does not implement the `Copy` trait
[00:27:53] error[E0382]: use of moved value
[00:27:53]    --> src/libcore/num/bignum.rs:402:21
[00:27:53]     |
[00:27:53] 100 | / macro_rules! define_bignum {
[00:27:53] 100 | / macro_rules! define_bignum {
[00:27:53] 101 | |     ($name:ident: type=$ty:ty, n=$n:expr) => (
[00:27:53] 102 | |         /// Stack-allocated arbitrary-precision (up to certain limit) integer.
[00:27:53] ...   |
[00:27:53] ...   |
[00:27:53] 402 | |                 for digit in &mut q.base[..] {
[00:27:53]     | |                     ^^^^^ value moved here, in previous iteration of loop
[00:27:53] 475 | |     )
[00:27:53] 476 | | }
[00:27:53]     | |_- in this expansion of `define_bignum!`
[00:27:53] ...
[00:27:53] ...
[00:27:53] 481 |   define_bignum!(Big32x40: type=Digit32, n=40);
[00:27:53]     |
[00:27:53]     |
[00:27:53]     = note: move occurs because value has type `&mut u32`, which does not implement the `Copy` trait
[00:27:53] error[E0382]: use of moved value
[00:27:53]    --> src/libcore/num/bignum.rs:405:21
[00:27:53]     |
[00:27:53] 100 | / macro_rules! define_bignum {
[00:27:53] 100 | / macro_rules! define_bignum {
[00:27:53] 101 | |     ($name:ident: type=$ty:ty, n=$n:expr) => (
[00:27:53] 102 | |         /// Stack-allocated arbitrary-precision (up to certain limit) integer.
[00:27:53] ...   |
[00:27:53] ...   |
[00:27:53] 405 | |                 for digit in &mut r.base[..] {
[00:27:53]     | |                     ^^^^^ value moved here, in previous iteration of loop
[00:27:53] 475 | |     )
[00:27:53] 476 | | }
[00:27:53]     | |_- in this expansion of `define_bignum!`
[00:27:53] ...
[00:27:53] ...
[00:27:53] 481 |   define_bignum!(Big32x40: type=Digit32, n=40);
[00:27:53]     |
[00:27:53]     |
[00:27:53]     = note: move occurs because value has type `&mut u32`, which does not implement the `Copy` trait
[00:27:53] error[E0382]: use of moved value
[00:27:53]    --> src/libcore/num/bignum.rs:402:21
[00:27:53]     |
[00:27:53] 100 | / macro_rules! define_bignum {
[00:27:53] 100 | / macro_rules! define_bignum {
[00:27:53] 101 | |     ($name:ident: type=$ty:ty, n=$n:expr) => (
[00:27:53] 102 | |         /// Stack-allocated arbitrary-precision (up to certain limit) integer.
[00:27:53] ...   |
[00:27:53] ...   |
[00:27:53] 402 | |                 for digit in &mut q.base[..] {
[00:27:53]     | |                     ^^^^^ value moved here, in previous iteration of loop
[00:27:53] 475 | |     )
[00:27:53] 476 | | }
[00:27:53]     | |_- in this expansion of `define_bignum!`
[00:27:53] ...
[00:27:53] ...
[00:27:53] 486 |       define_bignum!(Big8x3: type=u8, n=3);
[00:27:53]     |
[00:27:53]     |
[00:27:53]     = note: move occurs because value has type `&mut u8`, which does not implement the `Copy` trait
[00:27:53] error[E0382]: use of moved value
[00:27:53]    --> src/libcore/num/bignum.rs:405:21
[00:27:53]     |
[00:27:53] 100 | / macro_rules! define_bignum {
[00:27:53] 100 | / macro_rules! define_bignum {
[00:27:53] 101 | |     ($name:ident: type=$ty:ty, n=$n:expr) => (
[00:27:53] 102 | |         /// Stack-allocated arbitrary-precision (up to certain limit) integer.
[00:27:53] ...   |
[00:27:53] ...   |
[00:27:53] 405 | |                 for digit in &mut r.base[..] {
[00:27:53]     | |                     ^^^^^ value moved here, in previous iteration of loop
[00:27:53] 475 | |     )
[00:27:53] 476 | | }
[00:27:53]     | |_- in this expansion of `define_bignum!`
[00:27:53] ...
[00:27:53] ...
[00:27:53] 486 |       define_bignum!(Big8x3: type=u8, n=3);
[00:27:53]     |
[00:27:53]     |
[00:27:53]     = note: move occurs because value has type `&mut u8`, which does not implement the `Copy` trait
[00:27:54] error[E0381]: use of possibly uninitialized variable: `_`
[00:27:54]    --> src/libcore/cell.rs:821:15
[00:27:54]     |
[00:27:54]     |
[00:27:54] 821 |         match BorrowRef::new(&self.borrow) {
[00:27:54]     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ use of possibly uninitialized value
[00:27:54] error[E0381]: use of possibly uninitialized variable: `_`
[00:27:54]    --> src/libcore/cell.rs:899:15
[00:27:54]     |
[00:27:54]     |
[00:27:54] 899 |         match BorrowRefMut::new(&self.borrow) {
[00:27:54]     |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ use of possibly uninitialized value
[00:27:55] error[E0382]: use of moved value
[00:27:55]     --> src/libcore/slice/mod.rs:2520:13
[00:27:55]      |
[00:27:55] 2520 |         for byte in self {
[00:27:55] 2520 |         for byte in self {
[00:27:55]      |             ^^^^ value moved here, in previous iteration of loop
[00:27:55]      |
[00:27:55]      = note: move occurs because value has type `&mut u8`, which does not implement the `Copy` trait
[00:27:55] error[E0382]: use of moved value
[00:27:55]     --> src/libcore/slice/mod.rs:2537:13
[00:27:55]      |
[00:27:55] 2537 |         for byte in self {
[00:27:55] 2537 |         for byte in self {
[00:27:55]      |             ^^^^ value moved here, in previous iteration of loop
[00:27:55]      |
[00:27:55]      = note: move occurs because value has type `&mut u8`, which does not implement the `Copy` trait
[00:27:55] error[E0382]: use of moved value
[00:27:55]     --> src/libcore/slice/mod.rs:3158:32
[00:27:55]      |
[00:27:55] 3015 | / macro_rules! iterator {
[00:27:55] 3015 | / macro_rules! iterator {
[00:27:55] 3016 | |     (
[00:27:55] 3017 | |         struct $name:ident -> $ptr:ty,
[00:27:55] 3018 | |         $elem:ty,
[00:27:55] 3158 | |                 while let Some(x) = self.next() {
[00:27:55] 3158 | |                 while let Some(x) = self.next() {
[00:27:55]      | |                                ^ value moved here, in previous iteration of loop
[00:27:55] 3261 | |     }
[00:27:55] 3262 | | }
[00:27:55]      | |_- in this expansion of `iterator!`
[00:27:55] ...
[00:27:55] ...
[00:27:55] 3477 |   iterator!{struct IterMut -> *mut T, &'a mut T, mut, {mut}, {}}
[00:27:55]      |
[00:27:55]      |
[00:27:55]      = note: move occurs because value has type `&mut T`, which does not implement the `Copy` trait
[00:27:55] error[E0382]: use of moved value
[00:27:55]     --> src/libcore/slice/mod.rs:3249:32
[00:27:55]      |
[00:27:55] 3015 | / macro_rules! iterator {
[00:27:55] 3015 | / macro_rules! iterator {
[00:27:55] 3016 | |     (
[00:27:55] 3017 | |         struct $name:ident -> $ptr:ty,
[00:27:55] 3018 | |         $elem:ty,
[00:27:55] 3249 | |                 while let Some(x) = self.next_back() {
[00:27:55] 3249 | |                 while let Some(x) = self.next_back() {
[00:27:55]      | |                                ^ value moved here, in previous iteration of loop
[00:27:55] 3261 | |     }
[00:27:55] 3262 | | }
[00:27:55]      | |_- in this expansion of `iterator!`
[00:27:55] ...
[00:27:55] ...
[00:27:55] 3477 |   iterator!{struct IterMut -> *mut T, &'a mut T, mut, {mut}, {}}
[00:27:55]      |
[00:27:55]      |
[00:27:55]      = note: move occurs because value has type `&mut T`, which does not implement the `Copy` trait
[00:27:56] error[E0382]: use of moved value
[00:27:56]     --> src/libcore/fmt/mod.rs:1397:25
[00:27:56]      |
[00:27:56]      |
[00:27:56] 1397 |                     for c in s[..len].iter_mut().rev() {
[00:27:56]      |                         ^ value moved here, in previous iteration of loop
[00:27:56]      |
[00:27:56]      = note: move occurs because value has type `&mut u8`, which does not implement the `Copy` trait
[00:27:56] error[E0382]: use of moved value
[00:27:56] error[E0382]: use of moved value
[00:27:56]   --> src/libcore/fmt/num.rs:60:17
[00:27:56]    |
[00:27:56] 60 |             for byte in buf.iter_mut().rev() {
[00:27:56]    |                 ^^^^ value moved here, in previous iteration of loop
[00:27:56]    |
[00:27:56]    = note: move occurs because value has type `&mut mem::MaybeUninit<u8>`, which does not implement the `Copy` trait
[00:27:56] error[E0382]: use of moved value
[00:27:56] error[E0382]: use of moved value
[00:27:56]   --> src/libcore/fmt/num.rs:72:17
[00:27:56]    |
[00:27:56] 72 |             for byte in buf.iter_mut().rev() {
[00:27:56]    |                 ^^^^ value moved here, in previous iteration of loop
[00:27:56]    |
[00:27:56]    = note: move occurs because value has type `&mut mem::MaybeUninit<u8>`, which does not implement the `Copy` trait
[00:27:58] error: aborting due to 20 previous errors
[00:27:58] 
[00:27:58] Some errors have detailed explanations: E0381, E0382.
[00:27:58] For more information about an error, try `rustc --explain E0381`.
---
travis_time:end:02ba651a:start=1558718558630482420,finish=1558718558636830412,duration=6347992
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:04345324
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:02e6c09c
travis_time:start:02e6c09c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2100f086
$ dmesg | grep -i kill
