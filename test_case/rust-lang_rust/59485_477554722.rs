plain
travis_time:end:2d520126:start=1553770411608936469,finish=1553770412564953385,duration=956016916
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:28:46]    Compiling backtrace-sys v0.1.27
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/num/mod.rs:18:39
[00:28:47]     |
[00:28:47] 12  | / macro_rules! impl_nonzero_fmt {
[00:28:47] 13  | |     ( #[$stability: meta] ( $( $Trait: ident ),+ ) for $Ty: ident ) => {
[00:28:47] 14  | |         $(
[00:28:47] 15  | |             #[$stability]
[00:28:47] 18  | |                 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]     | |                                       ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...   |
[00:28:47] 23  | |     }
[00:28:47] 23  | |     }
[00:28:47] 24  | | }
[00:28:47]     | |_- in this expansion of `impl_nonzero_fmt!`
[00:28:47] 33  | / macro_rules! nonzero_integers {
[00:28:47] 33  | / macro_rules! nonzero_integers {
[00:28:47] 34  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
[00:28:47] 36  | |             doc_comment! {
[00:28:47] ...   |
[00:28:47] ...   |
[00:28:47] 93  | /             impl_nonzero_fmt! {
[00:28:47] 94  |                   #[$stability] (Debug, Display, Binary, Octal, LowerHex, UpperHex) for $Ty
[00:28:47]     | |_____________- in this macro invocation
[00:28:47] 96  |           )+
[00:28:47] 97  | |     }
[00:28:47] 98  | | }
---
[00:28:47] 112 | |     #[stable(feature = "signed_nonzero", since = "1.34.0")] NonZeroIsize(isize);
[00:28:47] 113 | | }
[00:28:47]     | |_- in this macro invocation
[00:28:47]     |
[00:28:47]     = note: #[deny(elided_lifetimes_in_paths)] on by default
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/num/mod.rs:18:39
[00:28:47]     |
[00:28:47]     |
[00:28:47] 12  | / macro_rules! impl_nonzero_fmt {
[00:28:47] 13  | |     ( #[$stability: meta] ( $( $Trait: ident ),+ ) for $Ty: ident ) => {
[00:28:47] 14  | |         $(
[00:28:47] 15  | |             #[$stability]
[00:28:47] 18  | |                 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]     | |                                       ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...   |
[00:28:47] 23  | |     }
[00:28:47] 23  | |     }
[00:28:47] 24  | | }
[00:28:47]     | |_- in this expansion of `impl_nonzero_fmt!`
[00:28:47] 33  | / macro_rules! nonzero_integers {
[00:28:47] 33  | / macro_rules! nonzero_integers {
[00:28:47] 34  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
[00:28:47] 36  | |             doc_comment! {
[00:28:47] ...   |
[00:28:47] ...   |
[00:28:47] 93  | /             impl_nonzero_fmt! {
[00:28:47] 94  |                   #[$stability] (Debug, Display, Binary, Octal, LowerHex, UpperHex) for $Ty
[00:28:47]     | |_____________- in this macro invocation
[00:28:47] 96  |           )+
[00:28:47] 97  | |     }
[00:28:47] 98  | | }
---
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/num/mod.rs:18:39
[00:28:47]     |
[00:28:47] 12  | / macro_rules! impl_nonzero_fmt {
[00:28:47] 13  | |     ( #[$stability: meta] ( $( $Trait: ident ),+ ) for $Ty: ident ) => {
[00:28:47] 14  | |         $(
[00:28:47] 15  | |             #[$stability]
[00:28:47] 18  | |                 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]     | |                                       ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...   |
[00:28:47] 23  | |     }
[00:28:47] 23  | |     }
[00:28:47] 24  | | }
[00:28:47]     | |_- in this expansion of `impl_nonzero_fmt!`
[00:28:47] 33  | / macro_rules! nonzero_integers {
[00:28:47] 33  | / macro_rules! nonzero_integers {
[00:28:47] 34  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
[00:28:47] 36  | |             doc_comment! {
[00:28:47] ...   |
[00:28:47] ...   |
[00:28:47] 93  | /             impl_nonzero_fmt! {
[00:28:47] 94  |                   #[$stability] (Debug, Display, Binary, Octal, LowerHex, UpperHex) for $Ty
[00:28:47]     | |_____________- in this macro invocation
[00:28:47] 96  |           )+
[00:28:47] 97  | |     }
[00:28:47] 98  | | }
---
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/num/mod.rs:18:39
[00:28:47]     |
[00:28:47] 12  | / macro_rules! impl_nonzero_fmt {
[00:28:47] 13  | |     ( #[$stability: meta] ( $( $Trait: ident ),+ ) for $Ty: ident ) => {
[00:28:47] 14  | |         $(
[00:28:47] 15  | |             #[$stability]
[00:28:47] 18  | |                 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]     | |                                       ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...   |
[00:28:47] 23  | |     }
[00:28:47] 23  | |     }
[00:28:47] 24  | | }
[00:28:47]     | |_- in this expansion of `impl_nonzero_fmt!`
[00:28:47] 33  | / macro_rules! nonzero_integers {
[00:28:47] 33  | / macro_rules! nonzero_integers {
[00:28:47] 34  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
[00:28:47] 36  | |             doc_comment! {
[00:28:47] ...   |
[00:28:47] ...   |
[00:28:47] 93  | /             impl_nonzero_fmt! {
[00:28:47] 94  |                   #[$stability] (Debug, Display, Binary, Octal, LowerHex, UpperHex) for $Ty
[00:28:47]     | |_____________- in this macro invocation
[00:28:47] 96  |           )+
[00:28:47] 97  | |     }
[00:28:47] 98  | | }
---
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/num/mod.rs:18:39
[00:28:47]     |
[00:28:47] 12  | / macro_rules! impl_nonzero_fmt {
[00:28:47] 13  | |     ( #[$stability: meta] ( $( $Trait: ident ),+ ) for $Ty: ident ) => {
[00:28:47] 14  | |         $(
[00:28:47] 15  | |             #[$stability]
[00:28:47] 18  | |                 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]     | |                                       ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...   |
[00:28:47] 23  | |     }
[00:28:47] 23  | |     }
[00:28:47] 24  | | }
[00:28:47]     | |_- in this expansion of `impl_nonzero_fmt!`
[00:28:47] 33  | / macro_rules! nonzero_integers {
[00:28:47] 33  | / macro_rules! nonzero_integers {
[00:28:47] 34  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
[00:28:47] 36  | |             doc_comment! {
[00:28:47] ...   |
[00:28:47] ...   |
[00:28:47] 93  | /             impl_nonzero_fmt! {
[00:28:47] 94  |                   #[$stability] (Debug, Display, Binary, Octal, LowerHex, UpperHex) for $Ty
[00:28:47]     | |_____________- in this macro invocation
[00:28:47] 96  |           )+
[00:28:47] 97  | |     }
[00:28:47] 98  | | }
---
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/num/mod.rs:18:39
[00:28:47]     |
[00:28:47] 12  | / macro_rules! impl_nonzero_fmt {
[00:28:47] 13  | |     ( #[$stability: meta] ( $( $Trait: ident ),+ ) for $Ty: ident ) => {
[00:28:47] 14  | |         $(
[00:28:47] 15  | |             #[$stability]
[00:28:47] 18  | |                 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]     | |                                       ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...   |
[00:28:47] 23  | |     }
[00:28:47] 23  | |     }
[00:28:47] 24  | | }
[00:28:47]     | |_- in this expansion of `impl_nonzero_fmt!`
[00:28:47] 33  | / macro_rules! nonzero_integers {
[00:28:47] 33  | / macro_rules! nonzero_integers {
[00:28:47] 34  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
[00:28:47] 36  | |             doc_comment! {
[00:28:47] ...   |
[00:28:47] ...   |
[00:28:47] 93  | /             impl_nonzero_fmt! {
[00:28:47] 94  |                   #[$stability] (Debug, Display, Binary, Octal, LowerHex, UpperHex) for $Ty
[00:28:47]     | |_____________- in this macro invocation
[00:28:47] 96  |           )+
[00:28:47] 97  | |     }
[00:28:47] 98  | | }
---
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/num/mod.rs:18:39
[00:28:47]     |
[00:28:47] 12  | / macro_rules! impl_nonzero_fmt {
[00:28:47] 13  | |     ( #[$stability: meta] ( $( $Trait: ident ),+ ) for $Ty: ident ) => {
[00:28:47] 14  | |         $(
[00:28:47] 15  | |             #[$stability]
[00:28:47] 18  | |                 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]     | |                                       ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...   |
[00:28:47] 23  | |     }
[00:28:47] 23  | |     }
[00:28:47] 24  | | }
[00:28:47]     | |_- in this expansion of `impl_nonzero_fmt!`
[00:28:47] 33  | / macro_rules! nonzero_integers {
[00:28:47] 33  | / macro_rules! nonzero_integers {
[00:28:47] 34  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
[00:28:47] 36  | |             doc_comment! {
[00:28:47] ...   |
[00:28:47] ...   |
[00:28:47] 93  | /             impl_nonzero_fmt! {
[00:28:47] 94  |                   #[$stability] (Debug, Display, Binary, Octal, LowerHex, UpperHex) for $Ty
[00:28:47]     | |_____________- in this macro invocation
[00:28:47] 96  |           )+
[00:28:47] 97  | |     }
[00:28:47] 98  | | }
---
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/num/mod.rs:18:39
[00:28:47]     |
[00:28:47] 12  | / macro_rules! impl_nonzero_fmt {
[00:28:47] 13  | |     ( #[$stability: meta] ( $( $Trait: ident ),+ ) for $Ty: ident ) => {
[00:28:47] 14  | |         $(
[00:28:47] 15  | |             #[$stability]
[00:28:47] 18  | |                 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]     | |                                       ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...   |
[00:28:47] 23  | |     }
[00:28:47] 23  | |     }
[00:28:47] 24  | | }
[00:28:47]     | |_- in this expansion of `impl_nonzero_fmt!`
[00:28:47] 33  | / macro_rules! nonzero_integers {
[00:28:47] 33  | / macro_rules! nonzero_integers {
[00:28:47] 34  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
[00:28:47] 36  | |             doc_comment! {
[00:28:47] ...   |
[00:28:47] ...   |
[00:28:47] 93  | /             impl_nonzero_fmt! {
[00:28:47] 94  |                   #[$stability] (Debug, Display, Binary, Octal, LowerHex, UpperHex) for $Ty
[00:28:47]     | |_____________- in this macro invocation
[00:28:47] 96  |           )+
[00:28:47] 97  | |     }
[00:28:47] 98  | | }
---
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/num/mod.rs:18:39
[00:28:47]     |
[00:28:47] 12  | / macro_rules! impl_nonzero_fmt {
[00:28:47] 13  | |     ( #[$stability: meta] ( $( $Trait: ident ),+ ) for $Ty: ident ) => {
[00:28:47] 14  | |         $(
[00:28:47] 15  | |             #[$stability]
[00:28:47] 18  | |                 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]     | |                                       ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...   |
[00:28:47] 23  | |     }
[00:28:47] 23  | |     }
[00:28:47] 24  | | }
[00:28:47]     | |_- in this expansion of `impl_nonzero_fmt!`
[00:28:47] 33  | / macro_rules! nonzero_integers {
[00:28:47] 33  | / macro_rules! nonzero_integers {
[00:28:47] 34  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
[00:28:47] 36  | |             doc_comment! {
[00:28:47] ...   |
[00:28:47] ...   |
[00:28:47] 93  | /             impl_nonzero_fmt! {
[00:28:47] 94  |                   #[$stability] (Debug, Display, Binary, Octal, LowerHex, UpperHex) for $Ty
[00:28:47]     | |_____________- in this macro invocation
[00:28:47] 96  |           )+
[00:28:47] 97  | |     }
[00:28:47] 98  | | }
---
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/num/mod.rs:18:39
[00:28:47]     |
[00:28:47] 12  | / macro_rules! impl_nonzero_fmt {
[00:28:47] 13  | |     ( #[$stability: meta] ( $( $Trait: ident ),+ ) for $Ty: ident ) => {
[00:28:47] 14  | |         $(
[00:28:47] 15  | |             #[$stability]
[00:28:47] 18  | |                 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]     | |                                       ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...   |
[00:28:47] 23  | |     }
[00:28:47] 23  | |     }
[00:28:47] 24  | | }
[00:28:47]     | |_- in this expansion of `impl_nonzero_fmt!`
[00:28:47] 33  | / macro_rules! nonzero_integers {
[00:28:47] 33  | / macro_rules! nonzero_integers {
[00:28:47] 34  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
[00:28:47] 36  | |             doc_comment! {
[00:28:47] ...   |
[00:28:47] ...   |
[00:28:47] 93  | /             impl_nonzero_fmt! {
[00:28:47] 94  |                   #[$stability] (Debug, Display, Binary, Octal, LowerHex, UpperHex) for $Ty
[00:28:47]     | |_____________- in this macro invocation
[00:28:47] 96  |           )+
[00:28:47] 97  | |     }
[00:28:47] 98  | | }
---
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/num/mod.rs:18:39
[00:28:47]     |
[00:28:47] 12  | / macro_rules! impl_nonzero_fmt {
[00:28:47] 13  | |     ( #[$stability: meta] ( $( $Trait: ident ),+ ) for $Ty: ident ) => {
[00:28:47] 14  | |         $(
[00:28:47] 15  | |             #[$stability]
[00:28:47] 18  | |                 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]     | |                                       ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...   |
[00:28:47] 23  | |     }
[00:28:47] 23  | |     }
[00:28:47] 24  | | }
[00:28:47]     | |_- in this expansion of `impl_nonzero_fmt!`
[00:28:47] 33  | / macro_rules! nonzero_integers {
[00:28:47] 33  | / macro_rules! nonzero_integers {
[00:28:47] 34  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
[00:28:47] 36  | |             doc_comment! {
[00:28:47] ...   |
[00:28:47] ...   |
[00:28:47] 93  | /             impl_nonzero_fmt! {
[00:28:47] 94  |                   #[$stability] (Debug, Display, Binary, Octal, LowerHex, UpperHex) for $Ty
[00:28:47]     | |_____________- in this macro invocation
[00:28:47] 96  |           )+
[00:28:47] 97  | |     }
[00:28:47] 98  | | }
---
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/num/mod.rs:18:39
[00:28:47]     |
[00:28:47] 12  | / macro_rules! impl_nonzero_fmt {
[00:28:47] 13  | |     ( #[$stability: meta] ( $( $Trait: ident ),+ ) for $Ty: ident ) => {
[00:28:47] 14  | |         $(
[00:28:47] 15  | |             #[$stability]
[00:28:47] 18  | |                 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]     | |                                       ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...   |
[00:28:47] 23  | |     }
[00:28:47] 23  | |     }
[00:28:47] 24  | | }
[00:28:47]     | |_- in this expansion of `impl_nonzero_fmt!`
[00:28:47] 33  | / macro_rules! nonzero_integers {
[00:28:47] 33  | / macro_rules! nonzero_integers {
[00:28:47] 34  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
[00:28:47] 36  | |             doc_comment! {
[00:28:47] ...   |
[00:28:47] ...   |
[00:28:47] 93  | /             impl_nonzero_fmt! {
[00:28:47] 94  |                   #[$stability] (Debug, Display, Binary, Octal, LowerHex, UpperHex) for $Ty
[00:28:47]     | |_____________- in this macro invocation
[00:28:47] 96  |           )+
[00:28:47] 97  | |     }
[00:28:47] 98  | | }
---
[00:28:47] 184 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]     |                           ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]   --> src/libcore/num/dec2flt/parse.rs:47:34
[00:28:47]    |
[00:28:47] 47 | pub fn parse_decimal(s: &str) -> ParseResult {
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/num/dec2flt/mod.rs:199:27
[00:28:47] 199 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]     |                           ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/num/dec2flt/mod.rs:247:38
[00:28:47]     |
[00:28:47] 247 | fn convert<T: RawFloat>(mut decimal: Decimal) -> Result<T, ParseFloatError> {
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/num/dec2flt/mod.rs:284:27
[00:28:47]     |
[00:28:47] 284 | fn simplify(decimal: &mut Decimal) {
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/num/dec2flt/mod.rs:309:40
[00:28:47]     |
[00:28:47] 309 | fn bound_intermediate_digits(decimal: &Decimal, e: i64) -> u64 {
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/num/dec2flt/mod.rs:328:41
[00:28:47]     |
[00:28:47] 328 | fn trivial_cases<T: RawFloat>(decimal: &Decimal) -> Option<T> {
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/num/bignum.rs:462:35
[00:28:47]     |
[00:28:47]     |
[00:28:47] 100 | / macro_rules! define_bignum {
[00:28:47] 101 | |     ($name:ident: type=$ty:ty, n=$n:expr) => (
[00:28:47] 102 | |         /// Stack-allocated arbitrary-precision (up to certain limit) integer.
[00:28:47] ...   |
[00:28:47] ...   |
[00:28:47] 462 | |             fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
[00:28:47] ...   |
[00:28:47] 475 | |     )
[00:28:47] 476 | | }
[00:28:47]     | |_- in this expansion of `define_bignum!`
[00:28:47]     | |_- in this expansion of `define_bignum!`
[00:28:47] ...
[00:28:47] 481 |   define_bignum!(Big32x40: type=Digit32, n=40);
[00:28:47]     |   --------------------------------------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/num/bignum.rs:462:35
[00:28:47]     |
[00:28:47] 100 | / macro_rules! define_bignum {
[00:28:47] 100 | / macro_rules! define_bignum {
[00:28:47] 101 | |     ($name:ident: type=$ty:ty, n=$n:expr) => (
[00:28:47] 102 | |         /// Stack-allocated arbitrary-precision (up to certain limit) integer.
[00:28:47] ...   |
[00:28:47] ...   |
[00:28:47] 462 | |             fn fmt(&self, f: &mut ::fmt::Formatter) -> ::fmt::Result {
[00:28:47] ...   |
[00:28:47] 475 | |     )
[00:28:47] 476 | | }
[00:28:47]     | |_- in this expansion of `define_bignum!`
[00:28:47]     | |_- in this expansion of `define_bignum!`
[00:28:47] ...
[00:28:47] 486 |       define_bignum!(Big8x3: type=u8, n=3);
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/num/mod.rs:4408:29
[00:28:47]      |
---
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/ptr.rs:2627:35
[00:28:47]      |
[00:28:47] 2589 | / macro_rules! fnptr_impls_safety_abi {
[00:28:47] 2590 | |     ($FnTy: ty, $($Arg: ident),*) => {
[00:28:47] 2591 | |         #[stable(feature = "fnptr_impls", since = "1.4.0")]
[00:28:47] 2592 | |         impl<Ret, $($Arg),*> PartialEq for $FnTy {
[00:28:47] 2627 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 2638 | |     }
[00:28:47] 2638 | |     }
[00:28:47] 2639 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_safety_abi!`
[00:28:47] 2640 | 
[00:28:47] 2641 | / macro_rules! fnptr_impls_args {
[00:28:47] 2642 | |     ($($Arg: ident),+) => {
[00:28:47] 2643 | |         fnptr_impls_safety_abi! { extern "Rust" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] 2644 | |         fnptr_impls_safety_abi! { extern "C" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] ...    |
[00:28:47] 2652 | |         fnptr_impls_safety_abi! { extern "Rust" fn() -> Ret, }
[00:28:47] ...    |
[00:28:47] 2656 | |     };
[00:28:47] 2657 | | }
[00:28:47] 2657 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_args!`
[00:28:47] 2658 | 
[00:28:47] 2659 |   fnptr_impls_args! { }
[00:28:47]      |   --------------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/ptr.rs:2634:35
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2589 | / macro_rules! fnptr_impls_safety_abi {
[00:28:47] 2590 | |     ($FnTy: ty, $($Arg: ident),*) => {
[00:28:47] 2591 | |         #[stable(feature = "fnptr_impls", since = "1.4.0")]
[00:28:47] 2592 | |         impl<Ret, $($Arg),*> PartialEq for $FnTy {
[00:28:47] 2634 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 2638 | |     }
[00:28:47] 2638 | |     }
[00:28:47] 2639 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_safety_abi!`
[00:28:47] 2640 | 
[00:28:47] 2641 | / macro_rules! fnptr_impls_args {
[00:28:47] 2642 | |     ($($Arg: ident),+) => {
[00:28:47] 2643 | |         fnptr_impls_safety_abi! { extern "Rust" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] 2644 | |         fnptr_impls_safety_abi! { extern "C" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] ...    |
[00:28:47] 2652 | |         fnptr_impls_safety_abi! { extern "Rust" fn() -> Ret, }
[00:28:47] ...    |
[00:28:47] 2656 | |     };
[00:28:47] 2657 | | }
[00:28:47] 2657 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_args!`
[00:28:47] 2658 | 
[00:28:47] 2659 |   fnptr_impls_args! { }
[00:28:47]      |   --------------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/ptr.rs:2627:35
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2589 | / macro_rules! fnptr_impls_safety_abi {
[00:28:47] 2590 | |     ($FnTy: ty, $($Arg: ident),*) => {
[00:28:47] 2591 | |         #[stable(feature = "fnptr_impls", since = "1.4.0")]
[00:28:47] 2592 | |         impl<Ret, $($Arg),*> PartialEq for $FnTy {
[00:28:47] 2627 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 2638 | |     }
[00:28:47] 2638 | |     }
[00:28:47] 2639 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_safety_abi!`
[00:28:47] 2640 | 
[00:28:47] 2641 | / macro_rules! fnptr_impls_args {
[00:28:47] 2642 | |     ($($Arg: ident),+) => {
[00:28:47] 2643 | |         fnptr_impls_safety_abi! { extern "Rust" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] 2644 | |         fnptr_impls_safety_abi! { extern "C" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] ...    |
[00:28:47] 2653 | |         fnptr_impls_safety_abi! { extern "C" fn() -> Ret, }
[00:28:47] ...    |
[00:28:47] 2656 | |     };
[00:28:47] 2657 | | }
[00:28:47] 2657 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_args!`
[00:28:47] 2658 | 
[00:28:47] 2659 |   fnptr_impls_args! { }
[00:28:47]      |   --------------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/ptr.rs:2634:35
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2589 | / macro_rules! fnptr_impls_safety_abi {
[00:28:47] 2590 | |     ($FnTy: ty, $($Arg: ident),*) => {
[00:28:47] 2591 | |         #[stable(feature = "fnptr_impls", since = "1.4.0")]
[00:28:47] 2592 | |         impl<Ret, $($Arg),*> PartialEq for $FnTy {
[00:28:47] 2634 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 2638 | |     }
[00:28:47] 2638 | |     }
[00:28:47] 2639 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_safety_abi!`
[00:28:47] 2640 | 
[00:28:47] 2641 | / macro_rules! fnptr_impls_args {
[00:28:47] 2642 | |     ($($Arg: ident),+) => {
[00:28:47] 2643 | |         fnptr_impls_safety_abi! { extern "Rust" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] 2644 | |         fnptr_impls_safety_abi! { extern "C" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] ...    |
[00:28:47] 2653 | |         fnptr_impls_safety_abi! { extern "C" fn() -> Ret, }
[00:28:47] ...    |
[00:28:47] 2656 | |     };
[00:28:47] 2657 | | }
[00:28:47] 2657 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_args!`
[00:28:47] 2658 | 
[00:28:47] 2659 |   fnptr_impls_args! { }
[00:28:47]      |   --------------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/ptr.rs:2627:35
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2589 | / macro_rules! fnptr_impls_safety_abi {
[00:28:47] 2590 | |     ($FnTy: ty, $($Arg: ident),*) => {
[00:28:47] 2591 | |         #[stable(feature = "fnptr_impls", since = "1.4.0")]
[00:28:47] 2592 | |         impl<Ret, $($Arg),*> PartialEq for $FnTy {
[00:28:47] 2627 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 2638 | |     }
[00:28:47] 2638 | |     }
[00:28:47] 2639 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_safety_abi!`
[00:28:47] 2640 | 
[00:28:47] 2641 | / macro_rules! fnptr_impls_args {
[00:28:47] 2642 | |     ($($Arg: ident),+) => {
[00:28:47] 2643 | |         fnptr_impls_safety_abi! { extern "Rust" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] 2644 | |         fnptr_impls_safety_abi! { extern "C" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] ...    |
[00:28:47] 2654 | |         fnptr_impls_safety_abi! { unsafe extern "Rust" fn() -> Ret, }
[00:28:47]      | |         ------------------------------------------------------------- in this macro invocation
[00:28:47] 2655 | |         fnptr_impls_safety_abi! { unsafe extern "C" fn() -> Ret, }
[00:28:47] 2657 | | }
[00:28:47] 2657 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_args!`
[00:28:47] 2658 | 
[00:28:47] 2659 |   fnptr_impls_args! { }
[00:28:47]      |   --------------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/ptr.rs:2634:35
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2589 | / macro_rules! fnptr_impls_safety_abi {
[00:28:47] 2590 | |     ($FnTy: ty, $($Arg: ident),*) => {
[00:28:47] 2591 | |         #[stable(feature = "fnptr_impls", since = "1.4.0")]
[00:28:47] 2592 | |         impl<Ret, $($Arg),*> PartialEq for $FnTy {
[00:28:47] 2634 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 2638 | |     }
[00:28:47] 2638 | |     }
[00:28:47] 2639 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_safety_abi!`
[00:28:47] 2640 | 
[00:28:47] 2641 | / macro_rules! fnptr_impls_args {
[00:28:47] 2642 | |     ($($Arg: ident),+) => {
[00:28:47] 2643 | |         fnptr_impls_safety_abi! { extern "Rust" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] 2644 | |         fnptr_impls_safety_abi! { extern "C" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] ...    |
[00:28:47] 2654 | |         fnptr_impls_safety_abi! { unsafe extern "Rust" fn() -> Ret, }
[00:28:47]      | |         ------------------------------------------------------------- in this macro invocation
[00:28:47] 2655 | |         fnptr_impls_safety_abi! { unsafe extern "C" fn() -> Ret, }
[00:28:47] 2657 | | }
[00:28:47] 2657 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_args!`
[00:28:47] 2658 | 
[00:28:47] 2659 |   fnptr_impls_args! { }
[00:28:47]      |   --------------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/ptr.rs:2627:35
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2589 | / macro_rules! fnptr_impls_safety_abi {
[00:28:47] 2590 | |     ($FnTy: ty, $($Arg: ident),*) => {
[00:28:47] 2591 | |         #[stable(feature = "fnptr_impls", since = "1.4.0")]
[00:28:47] 2592 | |         impl<Ret, $($Arg),*> PartialEq for $FnTy {
[00:28:47] 2627 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 2638 | |     }
[00:28:47] 2638 | |     }
[00:28:47] 2639 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_safety_abi!`
[00:28:47] 2640 | 
[00:28:47] 2641 | / macro_rules! fnptr_impls_args {
[00:28:47] 2642 | |     ($($Arg: ident),+) => {
[00:28:47] 2643 | |         fnptr_impls_safety_abi! { extern "Rust" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] 2644 | |         fnptr_impls_safety_abi! { extern "C" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] ...    |
[00:28:47] 2655 | |         fnptr_impls_safety_abi! { unsafe extern "C" fn() -> Ret, }
[00:28:47] 2656 | |     };
[00:28:47] 2657 | | }
[00:28:47] 2657 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_args!`
[00:28:47] 2658 | 
[00:28:47] 2659 |   fnptr_impls_args! { }
[00:28:47]      |   --------------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/ptr.rs:2634:35
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2589 | / macro_rules! fnptr_impls_safety_abi {
[00:28:47] 2590 | |     ($FnTy: ty, $($Arg: ident),*) => {
[00:28:47] 2591 | |         #[stable(feature = "fnptr_impls", since = "1.4.0")]
[00:28:47] 2592 | |         impl<Ret, $($Arg),*> PartialEq for $FnTy {
[00:28:47] 2634 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 2638 | |     }
[00:28:47] 2638 | |     }
[00:28:47] 2639 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_safety_abi!`
[00:28:47] 2640 | 
[00:28:47] 2641 | / macro_rules! fnptr_impls_args {
[00:28:47] 2642 | |     ($($Arg: ident),+) => {
[00:28:47] 2643 | |         fnptr_impls_safety_abi! { extern "Rust" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] 2644 | |         fnptr_impls_safety_abi! { extern "C" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] ...    |
[00:28:47] 2655 | |         fnptr_impls_safety_abi! { unsafe extern "C" fn() -> Ret, }
[00:28:47] 2656 | |     };
[00:28:47] 2657 | | }
[00:28:47] 2657 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_args!`
[00:28:47] 2658 | 
[00:28:47] 2659 |   fnptr_impls_args! { }
[00:28:47]      |   --------------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/ptr.rs:2627:35
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2589 | / macro_rules! fnptr_impls_safety_abi {
[00:28:47] 2590 | |     ($FnTy: ty, $($Arg: ident),*) => {
[00:28:47] 2591 | |         #[stable(feature = "fnptr_impls", since = "1.4.0")]
[00:28:47] 2592 | |         impl<Ret, $($Arg),*> PartialEq for $FnTy {
[00:28:47] 2627 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 2638 | |     }
[00:28:47] 2638 | |     }
[00:28:47] 2639 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_safety_abi!`
[00:28:47] 2640 | 
[00:28:47] 2641 | / macro_rules! fnptr_impls_args {
[00:28:47] 2642 | |     ($($Arg: ident),+) => {
[00:28:47] 2643 | |         fnptr_impls_safety_abi! { extern "Rust" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47]      | |         ------------------------------------------------------------------------- in this macro invocation
[00:28:47] 2644 | |         fnptr_impls_safety_abi! { extern "C" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] 2656 | |     };
[00:28:47] 2657 | | }
[00:28:47] 2657 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_args!`
[00:28:47] ...
[00:28:47] 2660 |   fnptr_impls_args! { A }
[00:28:47]      |   ----------------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/ptr.rs:2634:35
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2589 | / macro_rules! fnptr_impls_safety_abi {
[00:28:47] 2590 | |     ($FnTy: ty, $($Arg: ident),*) => {
[00:28:47] 2591 | |         #[stable(feature = "fnptr_impls", since = "1.4.0")]
[00:28:47] 2592 | |         impl<Ret, $($Arg),*> PartialEq for $FnTy {
[00:28:47] 2634 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 2638 | |     }
[00:28:47] 2638 | |     }
[00:28:47] 2639 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_safety_abi!`
[00:28:47] 2640 | 
[00:28:47] 2641 | / macro_rules! fnptr_impls_args {
[00:28:47] 2642 | |     ($($Arg: ident),+) => {
[00:28:47] 2643 | |         fnptr_impls_safety_abi! { extern "Rust" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47]      | |         ------------------------------------------------------------------------- in this macro invocation
[00:28:47] 2644 | |         fnptr_impls_safety_abi! { extern "C" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] 2656 | |     };
[00:28:47] 2657 | | }
[00:28:47] 2657 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_args!`
[00:28:47] ...
[00:28:47] 2660 |   fnptr_impls_args! { A }
[00:28:47]      |   ----------------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/ptr.rs:2627:35
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2589 | / macro_rules! fnptr_impls_safety_abi {
[00:28:47] 2590 | |     ($FnTy: ty, $($Arg: ident),*) => {
[00:28:47] 2591 | |         #[stable(feature = "fnptr_impls", since = "1.4.0")]
[00:28:47] 2592 | |         impl<Ret, $($Arg),*> PartialEq for $FnTy {
[00:28:47] 2627 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 2638 | |     }
[00:28:47] 2638 | |     }
[00:28:47] 2639 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_safety_abi!`
[00:28:47] 2640 | 
[00:28:47] 2641 | / macro_rules! fnptr_impls_args {
[00:28:47] 2642 | |     ($($Arg: ident),+) => {
[00:28:47] 2643 | |         fnptr_impls_safety_abi! { extern "Rust" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] 2644 | |         fnptr_impls_safety_abi! { extern "C" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] ...    |
[00:28:47] 2656 | |     };
[00:28:47] 2657 | | }
[00:28:47] 2657 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_args!`
[00:28:47] ...
[00:28:47] 2660 |   fnptr_impls_args! { A }
[00:28:47]      |   ----------------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/ptr.rs:2634:35
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2589 | / macro_rules! fnptr_impls_safety_abi {
[00:28:47] 2590 | |     ($FnTy: ty, $($Arg: ident),*) => {
[00:28:47] 2591 | |         #[stable(feature = "fnptr_impls", since = "1.4.0")]
[00:28:47] 2592 | |         impl<Ret, $($Arg),*> PartialEq for $FnTy {
[00:28:47] 2634 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 2638 | |     }
[00:28:47] 2638 | |     }
[00:28:47] 2639 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_safety_abi!`
[00:28:47] 2640 | 
[00:28:47] 2641 | / macro_rules! fnptr_impls_args {
[00:28:47] 2642 | |     ($($Arg: ident),+) => {
[00:28:47] 2643 | |         fnptr_impls_safety_abi! { extern "Rust" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] 2644 | |         fnptr_impls_safety_abi! { extern "C" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] ...    |
[00:28:47] 2656 | |     };
[00:28:47] 2657 | | }
[00:28:47] 2657 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_args!`
[00:28:47] ...
[00:28:47] 2660 |   fnptr_impls_args! { A }
[00:28:47]      |   ----------------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/ptr.rs:2627:35
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2589 | / macro_rules! fnptr_impls_safety_abi {
[00:28:47] 2590 | |     ($FnTy: ty, $($Arg: ident),*) => {
[00:28:47] 2591 | |         #[stable(feature = "fnptr_impls", since = "1.4.0")]
[00:28:47] 2592 | |         impl<Ret, $($Arg),*> PartialEq for $FnTy {
[00:28:47] 2627 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 2638 | |     }
[00:28:47] 2638 | |     }
[00:28:47] 2639 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_safety_abi!`
[00:28:47] 2640 | 
[00:28:47] 2641 | / macro_rules! fnptr_impls_args {
[00:28:47] 2642 | |     ($($Arg: ident),+) => {
[00:28:47] 2643 | |         fnptr_impls_safety_abi! { extern "Rust" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] 2644 | |         fnptr_impls_safety_abi! { extern "C" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] 2645 | |         fnptr_impls_safety_abi! { extern "C" fn($($Arg),* , ...) -> Ret, $($Arg),* }
[00:28:47] ...    |
[00:28:47] 2656 | |     };
[00:28:47] 2657 | | }
[00:28:47] 2657 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_args!`
[00:28:47] ...
[00:28:47] 2660 |   fnptr_impls_args! { A }
[00:28:47]      |   ----------------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/ptr.rs:2634:35
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2589 | / macro_rules! fnptr_impls_safety_abi {
[00:28:47] 2590 | |     ($FnTy: ty, $($Arg: ident),*) => {
[00:28:47] 2591 | |         #[stable(feature = "fnptr_impls", since = "1.4.0")]
[00:28:47] 2592 | |         impl<Ret, $($Arg),*> PartialEq for $FnTy {
[00:28:47] 2634 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 2638 | |     }
[00:28:47] 2638 | |     }
[00:28:47] 2639 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_safety_abi!`
[00:28:47] 2640 | 
[00:28:47] 2641 | / macro_rules! fnptr_impls_args {
[00:28:47] 2642 | |     ($($Arg: ident),+) => {
[00:28:47] 2643 | |         fnptr_impls_safety_abi! { extern "Rust" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] 2644 | |         fnptr_impls_safety_abi! { extern "C" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] 2645 | |         fnptr_impls_safety_abi! { extern "C" fn($($Arg),* , ...) -> Ret, $($Arg),* }
[00:28:47] ...    |
[00:28:47] 2656 | |     };
[00:28:47] 2657 | | }
[00:28:47] 2657 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_args!`
[00:28:47] ...
[00:28:47] 2660 |   fnptr_impls_args! { A }
[00:28:47]      |   ----------------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/ptr.rs:2627:35
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2589 | / macro_rules! fnptr_impls_safety_abi {
[00:28:47] 2590 | |     ($FnTy: ty, $($Arg: ident),*) => {
[00:28:47] 2591 | |         #[stable(feature = "fnptr_impls", since = "1.4.0")]
[00:28:47] 2592 | |         impl<Ret, $($Arg),*> PartialEq for $FnTy {
[00:28:47] 2627 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 2638 | |     }
[00:28:47] 2638 | |     }
[00:28:47] 2639 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_safety_abi!`
[00:28:47] 2640 | 
[00:28:47] 2641 | / macro_rules! fnptr_impls_args {
[00:28:47] 2642 | |     ($($Arg: ident),+) => {
[00:28:47] 2643 | |         fnptr_impls_safety_abi! { extern "Rust" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] 2644 | |         fnptr_impls_safety_abi! { extern "C" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] 2645 | |         fnptr_impls_safety_abi! { extern "C" fn($($Arg),* , ...) -> Ret, $($Arg),* }
[00:28:47] 2646 | |         fnptr_impls_safety_abi! { unsafe extern "Rust" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] ...    |
[00:28:47] 2656 | |     };
[00:28:47] 2657 | | }
[00:28:47] 2657 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_args!`
[00:28:47] ...
[00:28:47] 2660 |   fnptr_impls_args! { A }
[00:28:47]      |   ----------------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/ptr.rs:2634:35
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2589 | / macro_rules! fnptr_impls_safety_abi {
[00:28:47] 2590 | |     ($FnTy: ty, $($Arg: ident),*) => {
[00:28:47] 2591 | |         #[stable(feature = "fnptr_impls", since = "1.4.0")]
[00:28:47] 2592 | |         impl<Ret, $($Arg),*> PartialEq for $FnTy {
[00:28:47] 2634 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 2638 | |     }
[00:28:47] 2638 | |     }
[00:28:47] 2639 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_safety_abi!`
[00:28:47] 2640 | 
[00:28:47] 2641 | / macro_rules! fnptr_impls_args {
[00:28:47] 2642 | |     ($($Arg: ident),+) => {
[00:28:47] 2643 | |         fnptr_impls_safety_abi! { extern "Rust" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] 2644 | |         fnptr_impls_safety_abi! { extern "C" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] 2645 | |         fnptr_impls_safety_abi! { extern "C" fn($($Arg),* , ...) -> Ret, $($Arg),* }
[00:28:47] 2646 | |         fnptr_impls_safety_abi! { unsafe extern "Rust" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] ...    |
[00:28:47] 2656 | |     };
[00:28:47] 2657 | | }
[00:28:47] 2657 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_args!`
[00:28:47] ...
[00:28:47] 2660 |   fnptr_impls_args! { A }
[00:28:47]      |   ----------------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/ptr.rs:2627:35
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2589 | / macro_rules! fnptr_impls_safety_abi {
[00:28:47] 2590 | |     ($FnTy: ty, $($Arg: ident),*) => {
[00:28:47] 2591 | |         #[stable(feature = "fnptr_impls", since = "1.4.0")]
[00:28:47] 2592 | |         impl<Ret, $($Arg),*> PartialEq for $FnTy {
[00:28:47] 2627 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 2638 | |     }
[00:28:47] 2638 | |     }
[00:28:47] 2639 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_safety_abi!`
[00:28:47] 2640 | 
[00:28:47] 2641 | / macro_rules! fnptr_impls_args {
[00:28:47] 2642 | |     ($($Arg: ident),+) => {
[00:28:47] 2643 | |         fnptr_impls_safety_abi! { extern "Rust" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] 2644 | |         fnptr_impls_safety_abi! { extern "C" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] ...    |
[00:28:47] 2647 | |         fnptr_impls_safety_abi! { unsafe extern "C" fn($($Arg),*) -> Ret, $($Arg),* }
[00:28:47] ...    |
[00:28:47] 2656 | |     };
[00:28:47] 2657 | | }
[00:28:47] 2657 | | }
[00:28:47]      | |_- in this expansion of `fnptr_impls_args!`
[00:28:47] ...
[00:28:47] 2660 |   fnptr_impls_args! { A }
[00:28:47]      |   ----------------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/ptr.rs:2634:35
---
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/array.rs:187:39
[00:28:47]     |
[00:28:47] 108 | / macro_rules! array_impls {
[00:28:47] 109 | |     ($($N:expr)+) => {
[00:28:47] 111 | |             #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] ...   |
[00:28:47] 187 | |                 fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]     | |                                       ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47]     | |                                       ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...   |
[00:28:47] 255 | |     }
[00:28:47] 256 | | }
[00:28:47]     | |_- in this expansion of `array_impls!`
[00:28:47] 257 | 
[00:28:47] 258 | / array_impls! {
[00:28:47] 260 | |     10 11 12 13 14 15 16 17 18 19
[00:28:47] 261 | |     20 21 22 23 24 25 26 27 28 29
[00:28:47] 262 | |     30 31 32
[00:28:47] 263 | | }
---
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/sync/atomic.rs:1183:35
[00:28:47]      |
[00:28:47] 1123 | / macro_rules! atomic_int {
[00:28:47] 1124 | |     ($stable:meta,
[00:28:47] 1125 | |      $stable_cxchg:meta,
[00:28:47] 1126 | |      $stable_debug:meta,
[00:28:47] 1183 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 1876 | |     }
---
[00:28:47] 1881 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:28:47] 1882 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:28:47] 1883 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:28:47] ...    |
[00:28:47] 1893 | |     i8 AtomicI8 ATOMIC_I8_INIT
[00:28:47]      | |_- in this macro invocation
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/sync/atomic.rs:1183:35
[00:28:47]     --> src/libcore/sync/atomic.rs:1183:35
[00:28:47]      |
[00:28:47] 1123 | / macro_rules! atomic_int {
[00:28:47] 1124 | |     ($stable:meta,
[00:28:47] 1125 | |      $stable_cxchg:meta,
[00:28:47] 1126 | |      $stable_debug:meta,
[00:28:47] 1183 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 1876 | |     }
---
[00:28:47] 1897 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:28:47] 1898 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:28:47] 1899 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:28:47] ...    |
[00:28:47] 1909 | |     u8 AtomicU8 ATOMIC_U8_INIT
[00:28:47]      | |_- in this macro invocation
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/sync/atomic.rs:1183:35
[00:28:47]     --> src/libcore/sync/atomic.rs:1183:35
[00:28:47]      |
[00:28:47] 1123 | / macro_rules! atomic_int {
[00:28:47] 1124 | |     ($stable:meta,
[00:28:47] 1125 | |      $stable_cxchg:meta,
[00:28:47] 1126 | |      $stable_debug:meta,
[00:28:47] 1183 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 1876 | |     }
---
[00:28:47] 1913 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:28:47] 1914 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:28:47] 1915 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:28:47] ...    |
[00:28:47] 1925 | |     i16 AtomicI16 ATOMIC_I16_INIT
[00:28:47]      | |_- in this macro invocation
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/sync/atomic.rs:1183:35
[00:28:47]     --> src/libcore/sync/atomic.rs:1183:35
[00:28:47]      |
[00:28:47] 1123 | / macro_rules! atomic_int {
[00:28:47] 1124 | |     ($stable:meta,
[00:28:47] 1125 | |      $stable_cxchg:meta,
[00:28:47] 1126 | |      $stable_debug:meta,
[00:28:47] 1183 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 1876 | |     }
---
[00:28:47] 1929 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:28:47] 1930 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:28:47] 1931 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:28:47] ...    |
[00:28:47] 1941 | |     u16 AtomicU16 ATOMIC_U16_INIT
[00:28:47]      | |_- in this macro invocation
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/sync/atomic.rs:1183:35
[00:28:47]     --> src/libcore/sync/atomic.rs:1183:35
[00:28:47]      |
[00:28:47] 1123 | / macro_rules! atomic_int {
[00:28:47] 1124 | |     ($stable:meta,
[00:28:47] 1125 | |      $stable_cxchg:meta,
[00:28:47] 1126 | |      $stable_debug:meta,
[00:28:47] 1183 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 1876 | |     }
---
[00:28:47] 1945 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:28:47] 1946 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:28:47] 1947 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:28:47] ...    |
[00:28:47] 1957 | |     i32 AtomicI32 ATOMIC_I32_INIT
[00:28:47]      | |_- in this macro invocation
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/sync/atomic.rs:1183:35
[00:28:47]     --> src/libcore/sync/atomic.rs:1183:35
[00:28:47]      |
[00:28:47] 1123 | / macro_rules! atomic_int {
[00:28:47] 1124 | |     ($stable:meta,
[00:28:47] 1125 | |      $stable_cxchg:meta,
[00:28:47] 1126 | |      $stable_debug:meta,
[00:28:47] 1183 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 1876 | |     }
---
[00:28:47] 1961 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:28:47] 1962 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:28:47] 1963 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:28:47] ...    |
[00:28:47] 1973 | |     u32 AtomicU32 ATOMIC_U32_INIT
[00:28:47]      | |_- in this macro invocation
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/sync/atomic.rs:1183:35
[00:28:47]     --> src/libcore/sync/atomic.rs:1183:35
[00:28:47]      |
[00:28:47] 1123 | / macro_rules! atomic_int {
[00:28:47] 1124 | |     ($stable:meta,
[00:28:47] 1125 | |      $stable_cxchg:meta,
[00:28:47] 1126 | |      $stable_debug:meta,
[00:28:47] 1183 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 1876 | |     }
---
[00:28:47] 1977 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:28:47] 1978 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:28:47] 1979 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:28:47] ...    |
[00:28:47] 1989 | |     i64 AtomicI64 ATOMIC_I64_INIT
[00:28:47]      | |_- in this macro invocation
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/sync/atomic.rs:1183:35
[00:28:47]     --> src/libcore/sync/atomic.rs:1183:35
[00:28:47]      |
[00:28:47] 1123 | / macro_rules! atomic_int {
[00:28:47] 1124 | |     ($stable:meta,
[00:28:47] 1125 | |      $stable_cxchg:meta,
[00:28:47] 1126 | |      $stable_debug:meta,
[00:28:47] 1183 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 1876 | |     }
---
[00:28:47] 1993 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:28:47] 1994 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:28:47] 1995 | |     stable(feature = "integer_atomics_stable", since = "1.34.0"),
[00:28:47] ...    |
[00:28:47] 2005 | |     u64 AtomicU64 ATOMIC_U64_INIT
[00:28:47]      | |_- in this macro invocation
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/sync/atomic.rs:1183:35
[00:28:47]     --> src/libcore/sync/atomic.rs:1183:35
[00:28:47]      |
[00:28:47] 1123 | / macro_rules! atomic_int {
[00:28:47] 1124 | |     ($stable:meta,
[00:28:47] 1125 | |      $stable_cxchg:meta,
[00:28:47] 1126 | |      $stable_debug:meta,
[00:28:47] 1183 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 1876 | |     }
[00:28:47] 1876 | |     }
[00:28:47] 1877 | | }
[00:28:47]      | |_- in this expansion of `atomic_int!`
[00:28:47] ...
[00:28:47] 2052 | / atomic_int!{
[00:28:47] 2053 | |     stable(feature = "rust1", since = "1.0.0"),
[00:28:47] 2054 | |     stable(feature = "extended_compare_and_swap", since = "1.10.0"),
[00:28:47] 2055 | |     stable(feature = "atomic_debug", since = "1.3.0"),
[00:28:47] ...    |
[00:28:47] 2065 | |     isize AtomicIsize ATOMIC_ISIZE_INIT
[00:28:47]      | |_- in this macro invocation
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/sync/atomic.rs:1183:35
[00:28:47]     --> src/libcore/sync/atomic.rs:1183:35
[00:28:47]      |
[00:28:47] 1123 | / macro_rules! atomic_int {
[00:28:47] 1124 | |     ($stable:meta,
[00:28:47] 1125 | |      $stable_cxchg:meta,
[00:28:47] 1126 | |      $stable_debug:meta,
[00:28:47] 1183 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 1876 | |     }
[00:28:47] 1876 | |     }
[00:28:47] 1877 | | }
[00:28:47]      | |_- in this expansion of `atomic_int!`
[00:28:47] ...
[00:28:47] 2068 | / atomic_int!{
[00:28:47] 2069 | |     stable(feature = "rust1", since = "1.0.0"),
[00:28:47] 2070 | |     stable(feature = "extended_compare_and_swap", since = "1.10.0"),
[00:28:47] 2071 | |     stable(feature = "atomic_debug", since = "1.3.0"),
[00:28:47] 2081 | |     usize AtomicUsize ATOMIC_USIZE_INIT
[00:28:47] 2082 | | }
[00:28:47]      | |_- in this macro invocation
[00:28:47] 
---
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/cell.rs:794:29
[00:28:47]     |
[00:28:47] 794 |     pub fn borrow(&self) -> Ref<T> {
[00:28:47]     |                             ^^^^^^ help: indicate the anonymous lifetime: `Ref<'_, T>`
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/cell.rs:825:40
[00:28:47]     |
[00:28:47]     |
[00:28:47] 825 |     pub fn try_borrow(&self) -> Result<Ref<T>, BorrowError> {
[00:28:47]     |                                        ^^^^^^ help: indicate the anonymous lifetime: `Ref<'_, T>`
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/cell.rs:875:33
[00:28:47]     |
[00:28:47]     |
[00:28:47] 875 |     pub fn borrow_mut(&self) -> RefMut<T> {
[00:28:47]     |                                 ^^^^^^^^^ help: indicate the anonymous lifetime: `RefMut<'_, T>`
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/cell.rs:903:44
[00:28:47]     |
[00:28:47]     |
[00:28:47] 903 |     pub fn try_borrow_mut(&self) -> Result<RefMut<T>, BorrowMutError> {
[00:28:47]     |                                            ^^^^^^^^^ help: indicate the anonymous lifetime: `RefMut<'_, T>`
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/cell.rs:1213:27
[00:28:47]      |
[00:28:47] 1213 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
---
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]   --> src/libcore/panic.rs:89:38
[00:28:47]    |
[00:28:47] 89 |     pub fn message(&self) -> Option<&fmt::Arguments> {
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/panic.rs:118:39
[00:28:47]     |
[00:28:47]     |
[00:28:47] 118 |     pub fn location(&self) -> Option<&Location> {
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/panic.rs:127:35
[00:28:47]     |
---
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]   --> src/libcore/panicking.rs:68:23
[00:28:47]    |
[00:28:47] 68 | pub fn panic_fmt(fmt: fmt::Arguments, file_line_col: &(&'static str, u32, u32)) -> ! {
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]   --> src/libcore/panicking.rs:77:28
[00:28:47]    |
[00:28:47]    |
[00:28:47] 77 |         fn panic_impl(pi: &PanicInfo) -> !;
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/pin.rs:601:27
[00:28:47]     |
---
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/option.rs:539:27
[00:28:47]     |
[00:28:47] 539 |     pub fn iter(&self) -> Iter<T> {
[00:28:47]     |                           ^^^^^^^ help: indicate the anonymous lifetime: `Iter<'_, T>`
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/option.rs:560:35
[00:28:47]     |
[00:28:47]     |
[00:28:47] 560 |     pub fn iter_mut(&mut self) -> IterMut<T> {
[00:28:47]     |                                   ^^^^^^^^^^ help: indicate the anonymous lifetime: `IterMut<'_, T>`
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/result.rs:545:27
[00:28:47]     |
[00:28:47]     |
[00:28:47] 545 |     pub fn iter(&self) -> Iter<T> {
[00:28:47]     |                           ^^^^^^^ help: indicate the anonymous lifetime: `Iter<'_, T>`
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/result.rs:570:35
[00:28:47]     |
[00:28:47]     |
[00:28:47] 570 |     pub fn iter_mut(&mut self) -> IterMut<T> {
[00:28:47]     |                                   ^^^^^^^^^^ help: indicate the anonymous lifetime: `IterMut<'_, T>`
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]   --> src/libcore/ffi.rs:42:27
[00:28:47]    |
[00:28:47] 42 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47] 42 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]    |                           ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/ffi.rs:214:24
[00:28:47]     |
[00:28:47] 214 |     fn va_end(ap: &mut VaList);
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/ffi.rs:223:22
[00:28:47]     |
[00:28:47]     |
[00:28:47] 223 |     fn va_copy(src: &VaList) -> VaListImpl;
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/ffi.rs:227:52
[00:28:47]     |
[00:28:47]     |
[00:28:47] 227 |     fn va_arg<T: sealed_trait::VaArgSafe>(ap: &mut VaList) -> T;
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/slice/mod.rs:527:27
[00:28:47]     |
[00:28:47]     |
[00:28:47] 527 |     pub fn iter(&self) -> Iter<T> {
[00:28:47]     |                           ^^^^^^^ help: indicate the anonymous lifetime: `Iter<'_, T>`
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/slice/mod.rs:559:35
[00:28:47]     |
[00:28:47]     |
[00:28:47] 559 |     pub fn iter_mut(&mut self) -> IterMut<T> {
[00:28:47]     |                                   ^^^^^^^^^^ help: indicate the anonymous lifetime: `IterMut<'_, T>`
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/slice/mod.rs:606:43
[00:28:47]     |
[00:28:47]     |
[00:28:47] 606 |     pub fn windows(&self, size: usize) -> Windows<T> {
[00:28:47]     |                                           ^^^^^^^^^^ help: indicate the anonymous lifetime: `Windows<'_, T>`
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/slice/mod.rs:640:48
[00:28:47]     |
[00:28:47]     |
[00:28:47] 640 |     pub fn chunks(&self, chunk_size: usize) -> Chunks<T> {
[00:28:47]     |                                                ^^^^^^^^^ help: indicate the anonymous lifetime: `Chunks<'_, T>`
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/slice/mod.rs:678:56
[00:28:47]     |
[00:28:47]     |
[00:28:47] 678 |     pub fn chunks_mut(&mut self, chunk_size: usize) -> ChunksMut<T> {
[00:28:47]     |                                                        ^^^^^^^^^^^^ help: indicate the anonymous lifetime: `ChunksMut<'_, T>`
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/slice/mod.rs:715:54
[00:28:47]     |
[00:28:47]     |
[00:28:47] 715 |     pub fn chunks_exact(&self, chunk_size: usize) -> ChunksExact<T> {
[00:28:47]     |                                                      ^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `ChunksExact<'_, T>`
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/slice/mod.rs:760:62
[00:28:47]     |
[00:28:47]     |
[00:28:47] 760 |     pub fn chunks_exact_mut(&mut self, chunk_size: usize) -> ChunksExactMut<T> {
[00:28:47]     |                                                              ^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `ChunksExactMut<'_, T>`
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/slice/mod.rs:797:49
[00:28:47]     |
[00:28:47]     |
[00:28:47] 797 |     pub fn rchunks(&self, chunk_size: usize) -> RChunks<T> {
[00:28:47]     |                                                 ^^^^^^^^^^ help: indicate the anonymous lifetime: `RChunks<'_, T>`
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/slice/mod.rs:835:57
[00:28:47]     |
[00:28:47]     |
[00:28:47] 835 |     pub fn rchunks_mut(&mut self, chunk_size: usize) -> RChunksMut<T> {
[00:28:47]     |                                                         ^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `RChunksMut<'_, T>`
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/slice/mod.rs:874:55
[00:28:47]     |
[00:28:47]     |
[00:28:47] 874 |     pub fn rchunks_exact(&self, chunk_size: usize) -> RChunksExact<T> {
[00:28:47]     |                                                       ^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `RChunksExact<'_, T>`
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/slice/mod.rs:919:63
[00:28:47]     |
[00:28:47]     |
[00:28:47] 919 |     pub fn rchunks_exact_mut(&mut self, chunk_size: usize) -> RChunksExactMut<T> {
[00:28:47]     |                                                               ^^^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `RChunksExactMut<'_, T>`
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/slice/mod.rs:1045:40
[00:28:47]      |
[00:28:47]      |
[00:28:47] 1045 |     pub fn split<F>(&self, pred: F) -> Split<T, F>
[00:28:47]      |                                        ^^^^^^^^^^^ help: indicate the anonymous lifetime: `Split<'_, T, F>`
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/slice/mod.rs:1070:48
[00:28:47]      |
[00:28:47]      |
[00:28:47] 1070 |     pub fn split_mut<F>(&mut self, pred: F) -> SplitMut<T, F>
[00:28:47]      |                                                ^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `SplitMut<'_, T, F>`
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/slice/mod.rs:1105:41
[00:28:47]      |
[00:28:47]      |
[00:28:47] 1105 |     pub fn rsplit<F>(&self, pred: F) -> RSplit<T, F>
[00:28:47]      |                                         ^^^^^^^^^^^^ help: indicate the anonymous lifetime: `RSplit<'_, T, F>`
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/slice/mod.rs:1130:49
[00:28:47]      |
[00:28:47]      |
[00:28:47] 1130 |     pub fn rsplit_mut<F>(&mut self, pred: F) -> RSplitMut<T, F>
[00:28:47]      |                                                 ^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `RSplitMut<'_, T, F>`
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/slice/mod.rs:1157:51
[00:28:47]      |
[00:28:47]      |
[00:28:47] 1157 |     pub fn splitn<F>(&self, n: usize, pred: F) -> SplitN<T, F>
[00:28:47]      |                                                   ^^^^^^^^^^^^ help: indicate the anonymous lifetime: `SplitN<'_, T, F>`
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/slice/mod.rs:1187:59
[00:28:47]      |
[00:28:47]      |
[00:28:47] 1187 |     pub fn splitn_mut<F>(&mut self, n: usize, pred: F) -> SplitNMut<T, F>
[00:28:47]      |                                                           ^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `SplitNMut<'_, T, F>`
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/slice/mod.rs:1220:52
[00:28:47]      |
[00:28:47]      |
[00:28:47] 1220 |     pub fn rsplitn<F>(&self, n: usize, pred: F) -> RSplitN<T, F>
[00:28:47]      |                                                    ^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `RSplitN<'_, T, F>`
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/slice/mod.rs:1251:60
[00:28:47]      |
[00:28:47]      |
[00:28:47] 1251 |     pub fn rsplitn_mut<F>(&mut self, n: usize, pred: F) -> RSplitNMut<T, F>
[00:28:47]      |                                                            ^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `RSplitNMut<'_, T, F>`
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/slice/mod.rs:3139:27
[00:28:47]      |
[00:28:47] 3139 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
---
[00:28:47] 661 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]     |                           ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]   --> src/libcore/str/lossy.rs:22:29
[00:28:47]    |
[00:28:47] 22 |     pub fn chunks(&self) -> Utf8LossyChunksIter {
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/str/lossy.rs:142:27
[00:28:47] 142 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]     |                           ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/str/lossy.rs:168:27
[00:28:47] 168 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]     |                           ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
---
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 1019 | |     } => {}
[00:28:47] 1020 | | }
[00:28:47]      | |_- in this expansion of `generate_pattern_iterators!`
[00:28:47] ...
[00:28:47] 1105 | / generate_pattern_iterators! {
[00:28:47] 1106 | |     forward:
[00:28:47] 1107 | |         /// Created with the method [`split`].
[00:28:47] ...    |
[00:28:47] 1120 | |     delegate double ended;
[00:28:47] 1121 | | }
[00:28:47]      | |_- in this macro invocation
---
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 1019 | |     } => {}
[00:28:47] 1020 | | }
[00:28:47]      | |_- in this expansion of `generate_pattern_iterators!`
[00:28:47] ...
[00:28:47] 1105 | / generate_pattern_iterators! {
[00:28:47] 1106 | |     forward:
[00:28:47] 1107 | |         /// Created with the method [`split`].
[00:28:47] ...    |
[00:28:47] 1120 | |     delegate double ended;
[00:28:47] 1121 | | }
[00:28:47]      | |_- in this macro invocation
---
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 1019 | |     } => {}
[00:28:47] 1020 | | }
[00:28:47]      | |_- in this expansion of `generate_pattern_iterators!`
[00:28:47] ...
[00:28:47] 1123 | / generate_pattern_iterators! {
[00:28:47] 1124 | |     forward:
[00:28:47] 1125 | |         /// Created with the method [`split_terminator`].
[00:28:47] ...    |
[00:28:47] 1138 | |     delegate double ended;
[00:28:47] 1139 | | }
[00:28:47]      | |_- in this macro invocation
---
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 1019 | |     } => {}
[00:28:47] 1020 | | }
[00:28:47]      | |_- in this expansion of `generate_pattern_iterators!`
[00:28:47] ...
[00:28:47] 1123 | / generate_pattern_iterators! {
[00:28:47] 1124 | |     forward:
[00:28:47] 1125 | |         /// Created with the method [`split_terminator`].
[00:28:47] ...    |
[00:28:47] 1138 | |     delegate double ended;
[00:28:47] 1139 | | }
[00:28:47]      | |_- in this macro invocation
---
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 1019 | |     } => {}
[00:28:47] 1020 | | }
[00:28:47]      | |_- in this expansion of `generate_pattern_iterators!`
[00:28:47] ...
[00:28:47] 1183 | / generate_pattern_iterators! {
[00:28:47] 1184 | |     forward:
[00:28:47] 1185 | |         /// Created with the method [`splitn`].
[00:28:47] ...    |
[00:28:47] 1198 | |     delegate single ended;
[00:28:47] 1199 | | }
[00:28:47]      | |_- in this macro invocation
---
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 1019 | |     } => {}
[00:28:47] 1020 | | }
[00:28:47]      | |_- in this expansion of `generate_pattern_iterators!`
[00:28:47] ...
[00:28:47] 1183 | / generate_pattern_iterators! {
[00:28:47] 1184 | |     forward:
[00:28:47] 1185 | |         /// Created with the method [`splitn`].
[00:28:47] ...    |
[00:28:47] 1198 | |     delegate single ended;
[00:28:47] 1199 | | }
[00:28:47]      | |_- in this macro invocation
---
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 1019 | |     } => {}
[00:28:47] 1020 | | }
[00:28:47]      | |_- in this expansion of `generate_pattern_iterators!`
[00:28:47] ...
[00:28:47] 1234 | / generate_pattern_iterators! {
[00:28:47] 1235 | |     forward:
[00:28:47] 1236 | |         /// Created with the method [`match_indices`].
[00:28:47] ...    |
[00:28:47] 1249 | |     delegate double ended;
[00:28:47] 1250 | | }
[00:28:47]      | |_- in this macro invocation
---
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 1019 | |     } => {}
[00:28:47] 1020 | | }
[00:28:47]      | |_- in this expansion of `generate_pattern_iterators!`
[00:28:47] ...
[00:28:47] 1234 | / generate_pattern_iterators! {
[00:28:47] 1235 | |     forward:
[00:28:47] 1236 | |         /// Created with the method [`match_indices`].
[00:28:47] ...    |
[00:28:47] 1249 | |     delegate double ended;
[00:28:47] 1250 | | }
[00:28:47]      | |_- in this macro invocation
---
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 1019 | |     } => {}
[00:28:47] 1020 | | }
[00:28:47]      | |_- in this expansion of `generate_pattern_iterators!`
[00:28:47] ...
[00:28:47] 1287 | / generate_pattern_iterators! {
[00:28:47] 1288 | |     forward:
[00:28:47] 1289 | |         /// Created with the method [`matches`].
[00:28:47] ...    |
[00:28:47] 1302 | |     delegate double ended;
[00:28:47] 1303 | | }
[00:28:47]      | |_- in this macro invocation
---
[00:28:47]      | |                                   ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 1019 | |     } => {}
[00:28:47] 1020 | | }
[00:28:47]      | |_- in this expansion of `generate_pattern_iterators!`
[00:28:47] ...
[00:28:47] 1287 | / generate_pattern_iterators! {
[00:28:47] 1288 | |     forward:
[00:28:47] 1289 | |         /// Created with the method [`matches`].
[00:28:47] ...    |
[00:28:47] 1302 | |     delegate double ended;
[00:28:47] 1303 | | }
[00:28:47]      | |_- in this macro invocation
---
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/str/mod.rs:2617:35
[00:28:47]      |
[00:28:47] 2617 |     pub fn char_indices(&self) -> CharIndices {
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/str/mod.rs:2642:28
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2642 |     pub fn bytes(&self) -> Bytes {
[00:28:47]      |                            ^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/str/mod.rs:2685:39
[00:28:47]      |
[00:28:47] 2685 |     pub fn split_whitespace(&self) -> SplitWhitespace {
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/str/mod.rs:2726:45
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2726 |     pub fn split_ascii_whitespace(&self) -> SplitAsciiWhitespace {
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/str/mod.rs:2773:28
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2773 |     pub fn lines(&self) -> Lines {
[00:28:47]      |                            ^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/str/mod.rs:2782:32
[00:28:47]      |
[00:28:47] 2782 |     pub fn lines_any(&self) -> LinesAny {
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/str/mod.rs:2801:35
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2801 |     pub fn encode_utf16(&self) -> EncodeUtf16 {
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/str/mod.rs:4021:35
[00:28:47]      |
[00:28:47]      |
[00:28:47] 4021 |     pub fn escape_debug(&self) -> EscapeDebug {
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/str/mod.rs:4066:37
[00:28:47]      |
---
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/str/mod.rs:4344:35
[00:28:47]      |
[00:28:47] 4340 | / macro_rules! escape_types_impls {
[00:28:47] 4341 | |     ($( $Name: ident ),+) => {$(
[00:28:47] 4342 | |         #[stable(feature = "str_escape", since = "1.34.0")]
[00:28:47] 4343 | |         impl<'a> fmt::Display for $Name<'a> {
[00:28:47] 4344 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47] ...    |
[00:28:47] 4376 | |     )+}
[00:28:47] 4377 | | }
[00:28:47]      | |_- in this expansion of `escape_types_impls!`
[00:28:47]      | |_- in this expansion of `escape_types_impls!`
[00:28:47] 4378 | 
[00:28:47] 4379 |   escape_types_impls!(EscapeDebug, EscapeDefault, EscapeUnicode);
[00:28:47]      |   --------------------------------------------------------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/hash/mod.rs:507:27
[00:28:47]     |
[00:28:47] 507 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47] 507 |     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47]     |                           ^^^^^^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]   --> src/libcore/fmt/float.rs:14:40
[00:28:47]    |
[00:28:47] 14 |         let mut parts = MaybeUninit::<[flt2dec::Part; 4]>::uninit();
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]  --> src/libcore/fmt/float.rs:8:47
[00:28:47]   |
[00:28:47]   |
[00:28:47] 8 | fn float_to_decimal_common_exact<T>(fmt: &mut Formatter, num: &T,
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]   --> src/libcore/fmt/float.rs:36:40
[00:28:47]    |
[00:28:47]    |
[00:28:47] 36 |         let mut parts = MaybeUninit::<[flt2dec::Part; 4]>::uninit();
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]   --> src/libcore/fmt/float.rs:29:50
[00:28:47]    |
[00:28:47]    |
[00:28:47] 29 | fn float_to_decimal_common_shortest<T>(fmt: &mut Formatter, num: &T,
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]   --> src/libcore/fmt/float.rs:46:41
[00:28:47]    |
[00:28:47]    |
[00:28:47] 46 | fn float_to_decimal_common<T>(fmt: &mut Formatter, num: &T,
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]   --> src/libcore/fmt/float.rs:75:40
[00:28:47]    |
[00:28:47]    |
[00:28:47] 75 |         let mut parts = MaybeUninit::<[flt2dec::Part; 6]>::uninit();
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]   --> src/libcore/fmt/float.rs:68:51
[00:28:47]    |
[00:28:47]    |
[00:28:47] 68 | fn float_to_exponential_common_exact<T>(fmt: &mut Formatter, num: &T,
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]   --> src/libcore/fmt/float.rs:95:40
[00:28:47]    |
[00:28:47]    |
[00:28:47] 95 |         let mut parts = MaybeUninit::<[flt2dec::Part; 6]>::uninit();
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]   --> src/libcore/fmt/float.rs:87:54
[00:28:47]    |
[00:28:47]    |
[00:28:47] 87 | fn float_to_exponential_common_shortest<T>(fmt: &mut Formatter,
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/fmt/float.rs:105:45
[00:28:47]     |
[00:28:47]     |
[00:28:47] 105 | fn float_to_exponential_common<T>(fmt: &mut Formatter, num: &T, upper: bool) -> Result
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/fmt/float.rs:126:37
[00:28:47]     |
[00:28:47]     |
[00:28:47] 122 | / macro_rules! floating {
[00:28:47] 123 | |     ($ty:ident) => (
[00:28:47] 124 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 125 | |         impl Debug for $ty {
[00:28:47]     | |                                     ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...   |
[00:28:47] 151 | |     )
[00:28:47] 152 | | }
[00:28:47] 152 | | }
[00:28:47]     | |_- in this expansion of `floating!`
[00:28:47] 153 | 
[00:28:47] 154 |   floating! { f32 }
[00:28:47]     |   ----------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/fmt/float.rs:133:37
[00:28:47]     |
[00:28:47] 122 | / macro_rules! floating {
[00:28:47] 122 | / macro_rules! floating {
[00:28:47] 123 | |     ($ty:ident) => (
[00:28:47] 124 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 125 | |         impl Debug for $ty {
[00:28:47] 133 | |             fn fmt(&self, fmt: &mut Formatter) -> Result {
[00:28:47]     | |                                     ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...   |
[00:28:47] 151 | |     )
[00:28:47] 151 | |     )
[00:28:47] 152 | | }
[00:28:47]     | |_- in this expansion of `floating!`
[00:28:47] 153 | 
[00:28:47] 154 |   floating! { f32 }
[00:28:47]     |   ----------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/fmt/float.rs:140:37
[00:28:47]     |
[00:28:47] 122 | / macro_rules! floating {
[00:28:47] 122 | / macro_rules! floating {
[00:28:47] 123 | |     ($ty:ident) => (
[00:28:47] 124 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 125 | |         impl Debug for $ty {
[00:28:47] 140 | |             fn fmt(&self, fmt: &mut Formatter) -> Result {
[00:28:47]     | |                                     ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...   |
[00:28:47] 151 | |     )
[00:28:47] 151 | |     )
[00:28:47] 152 | | }
[00:28:47]     | |_- in this expansion of `floating!`
[00:28:47] 153 | 
[00:28:47] 154 |   floating! { f32 }
[00:28:47]     |   ----------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/fmt/float.rs:147:37
[00:28:47]     |
[00:28:47] 122 | / macro_rules! floating {
[00:28:47] 122 | / macro_rules! floating {
[00:28:47] 123 | |     ($ty:ident) => (
[00:28:47] 124 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 125 | |         impl Debug for $ty {
[00:28:47] 147 | |             fn fmt(&self, fmt: &mut Formatter) -> Result {
[00:28:47]     | |                                     ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...   |
[00:28:47] 151 | |     )
[00:28:47] 151 | |     )
[00:28:47] 152 | | }
[00:28:47]     | |_- in this expansion of `floating!`
[00:28:47] 153 | 
[00:28:47] 154 |   floating! { f32 }
[00:28:47]     |   ----------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/fmt/float.rs:126:37
[00:28:47]     |
[00:28:47] 122 | / macro_rules! floating {
[00:28:47] 122 | / macro_rules! floating {
[00:28:47] 123 | |     ($ty:ident) => (
[00:28:47] 124 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 125 | |         impl Debug for $ty {
[00:28:47]     | |                                     ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...   |
[00:28:47] 151 | |     )
[00:28:47] 152 | | }
[00:28:47] 152 | | }
[00:28:47]     | |_- in this expansion of `floating!`
[00:28:47] ...
[00:28:47] 155 |   floating! { f64 }
[00:28:47]     |   ----------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/fmt/float.rs:133:37
[00:28:47]     |
[00:28:47] 122 | / macro_rules! floating {
[00:28:47] 122 | / macro_rules! floating {
[00:28:47] 123 | |     ($ty:ident) => (
[00:28:47] 124 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 125 | |         impl Debug for $ty {
[00:28:47] 133 | |             fn fmt(&self, fmt: &mut Formatter) -> Result {
[00:28:47]     | |                                     ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...   |
[00:28:47] 151 | |     )
[00:28:47] 151 | |     )
[00:28:47] 152 | | }
[00:28:47]     | |_- in this expansion of `floating!`
[00:28:47] ...
[00:28:47] 155 |   floating! { f64 }
[00:28:47]     |   ----------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/fmt/float.rs:140:37
[00:28:47]     |
[00:28:47] 122 | / macro_rules! floating {
[00:28:47] 122 | / macro_rules! floating {
[00:28:47] 123 | |     ($ty:ident) => (
[00:28:47] 124 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 125 | |         impl Debug for $ty {
[00:28:47] 140 | |             fn fmt(&self, fmt: &mut Formatter) -> Result {
[00:28:47]     | |                                     ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...   |
[00:28:47] 151 | |     )
[00:28:47] 151 | |     )
[00:28:47] 152 | | }
[00:28:47]     | |_- in this expansion of `floating!`
[00:28:47] ...
[00:28:47] 155 |   floating! { f64 }
[00:28:47]     |   ----------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/fmt/float.rs:147:37
[00:28:47]     |
[00:28:47] 122 | / macro_rules! floating {
[00:28:47] 122 | / macro_rules! floating {
[00:28:47] 123 | |     ($ty:ident) => (
[00:28:47] 124 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 125 | |         impl Debug for $ty {
[00:28:47] 147 | |             fn fmt(&self, fmt: &mut Formatter) -> Result {
[00:28:47]     | |                                     ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...   |
[00:28:47] 151 | |     )
[00:28:47] 151 | |     )
[00:28:47] 152 | | }
[00:28:47]     | |_- in this expansion of `floating!`
[00:28:47] ...
[00:28:47] 155 |   floating! { f64 }
[00:28:47]     |   ----------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]   --> src/libcore/fmt/num.rs:49:49
[00:28:47]    |
[00:28:47] 49 |     fn fmt_int<T: Int>(&self, mut x: T, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/fmt/num.rs:134:35
[00:28:47] 130 | / macro_rules! int_base {
[00:28:47] 130 | / macro_rules! int_base {
[00:28:47] 131 | |     ($Trait:ident for $T:ident as $U:ident -> $Radix:ident) => {
[00:28:47] 132 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 133 | |         impl fmt::$Trait for $T {
[00:28:47] 134 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47] ...   |
[00:28:47] 138 | |     }
[00:28:47] 139 | | }
[00:28:47]     | |_- in this expansion of `int_base!`
[00:28:47]     | |_- in this expansion of `int_base!`
[00:28:47] ...
[00:28:47] 159 | / macro_rules! integer {
[00:28:47] 160 | |     ($Int:ident, $Uint:ident) => {
[00:28:47] 161 | |         int_base! { Binary   for $Int as $Uint  -> Binary }
[00:28:47]     | |         --------------------------------------------------- in this macro invocation
[00:28:47] 162 | |         int_base! { Octal    for $Int as $Uint  -> Octal }
[00:28:47] 172 | |     }
[00:28:47] 173 | | }
[00:28:47]     | |_- in this expansion of `integer!`
[00:28:47]     | |_- in this expansion of `integer!`
[00:28:47] 174 |   integer! { isize, usize }
[00:28:47]     |   ------------------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/fmt/num.rs:134:35
[00:28:47] 130 | / macro_rules! int_base {
[00:28:47] 130 | / macro_rules! int_base {
[00:28:47] 131 | |     ($Trait:ident for $T:ident as $U:ident -> $Radix:ident) => {
[00:28:47] 132 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 133 | |         impl fmt::$Trait for $T {
[00:28:47] 134 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47] ...   |
[00:28:47] 138 | |     }
[00:28:47] 139 | | }
[00:28:47]     | |_- in this expansion of `int_base!`
[00:28:47]     | |_- in this expansion of `int_base!`
[00:28:47] ...
[00:28:47] 159 | / macro_rules! integer {
[00:28:47] 160 | |     ($Int:ident, $Uint:ident) => {
[00:28:47] 161 | |         int_base! { Binary   for $Int as $Uint  -> Binary }
[00:28:47] 162 | |         int_base! { Octal    for $Int as $Uint  -> Octal }
[00:28:47] ...   |
[00:28:47] 172 | |     }
[00:28:47] 173 | | }
[00:28:47]     | |_- in this expansion of `integer!`
[00:28:47]     | |_- in this expansion of `integer!`
[00:28:47] 174 |   integer! { isize, usize }
[00:28:47]     |   ------------------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/fmt/num.rs:134:35
[00:28:47] 130 | / macro_rules! int_base {
[00:28:47] 130 | / macro_rules! int_base {
[00:28:47] 131 | |     ($Trait:ident for $T:ident as $U:ident -> $Radix:ident) => {
[00:28:47] 132 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 133 | |         impl fmt::$Trait for $T {
[00:28:47] 134 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47] ...   |
[00:28:47] 138 | |     }
[00:28:47] 139 | | }
[00:28:47]     | |_- in this expansion of `int_base!`
[00:28:47]     | |_- in this expansion of `int_base!`
[00:28:47] ...
[00:28:47] 159 | / macro_rules! integer {
[00:28:47] 160 | |     ($Int:ident, $Uint:ident) => {
[00:28:47] 161 | |         int_base! { Binary   for $Int as $Uint  -> Binary }
[00:28:47] 162 | |         int_base! { Octal    for $Int as $Uint  -> Octal }
[00:28:47] 163 | |         int_base! { LowerHex for $Int as $Uint  -> LowerHex }
[00:28:47] ...   |
[00:28:47] 172 | |     }
[00:28:47] 173 | | }
[00:28:47]     | |_- in this expansion of `integer!`
[00:28:47]     | |_- in this expansion of `integer!`
[00:28:47] 174 |   integer! { isize, usize }
[00:28:47]     |   ------------------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/fmt/num.rs:134:35
[00:28:47] 130 | / macro_rules! int_base {
[00:28:47] 130 | / macro_rules! int_base {
[00:28:47] 131 | |     ($Trait:ident for $T:ident as $U:ident -> $Radix:ident) => {
[00:28:47] 132 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 133 | |         impl fmt::$Trait for $T {
[00:28:47] 134 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47] ...   |
[00:28:47] 138 | |     }
[00:28:47] 139 | | }
[00:28:47]     | |_- in this expansion of `int_base!`
[00:28:47]     | |_- in this expansion of `int_base!`
[00:28:47] ...
[00:28:47] 159 | / macro_rules! integer {
[00:28:47] 160 | |     ($Int:ident, $Uint:ident) => {
[00:28:47] 161 | |         int_base! { Binary   for $Int as $Uint  -> Binary }
[00:28:47] 162 | |         int_base! { Octal    for $Int as $Uint  -> Octal }
[00:28:47] 163 | |         int_base! { LowerHex for $Int as $Uint  -> LowerHex }
[00:28:47] 164 | |         int_base! { UpperHex for $Int as $Uint  -> UpperHex }
[00:28:47] ...   |
[00:28:47] 172 | |     }
[00:28:47] 173 | | }
[00:28:47]     | |_- in this expansion of `integer!`
[00:28:47]     | |_- in this expansion of `integer!`
[00:28:47] 174 |   integer! { isize, usize }
[00:28:47]     |   ------------------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/fmt/num.rs:146:35
[00:28:47] 141 | / macro_rules! debug {
[00:28:47] 142 | |     ($T:ident) => {
[00:28:47] 143 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 143 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 144 | |         impl fmt::Debug for $T {
[00:28:47] 145 | |             #[inline]
[00:28:47] 146 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47] ...   |
[00:28:47] 156 | |     }
[00:28:47] 157 | | }
[00:28:47]     | |_- in this expansion of `debug!`
[00:28:47]     | |_- in this expansion of `debug!`
[00:28:47] 158 | 
[00:28:47] 159 | / macro_rules! integer {
[00:28:47] 160 | |     ($Int:ident, $Uint:ident) => {
[00:28:47] 161 | |         int_base! { Binary   for $Int as $Uint  -> Binary }
[00:28:47] 162 | |         int_base! { Octal    for $Int as $Uint  -> Octal }
[00:28:47] ...   |
[00:28:47] 165 | |         debug! { $Int }
[00:28:47] ...   |
[00:28:47] 172 | |     }
[00:28:47] 173 | | }
[00:28:47]     | |_- in this expansion of `integer!`
[00:28:47]     | |_- in this expansion of `integer!`
[00:28:47] 174 |   integer! { isize, usize }
[00:28:47]     |   ------------------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/fmt/num.rs:134:35
[00:28:47] 130 | / macro_rules! int_base {
[00:28:47] 130 | / macro_rules! int_base {
[00:28:47] 131 | |     ($Trait:ident for $T:ident as $U:ident -> $Radix:ident) => {
[00:28:47] 132 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 133 | |         impl fmt::$Trait for $T {
[00:28:47] 134 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47] ...   |
[00:28:47] 138 | |     }
[00:28:47] 139 | | }
[00:28:47]     | |_- in this expansion of `int_base!`
[00:28:47]     | |_- in this expansion of `int_base!`
[00:28:47] ...
[00:28:47] 159 | / macro_rules! integer {
[00:28:47] 160 | |     ($Int:ident, $Uint:ident) => {
[00:28:47] 161 | |         int_base! { Binary   for $Int as $Uint  -> Binary }
[00:28:47] 162 | |         int_base! { Octal    for $Int as $Uint  -> Octal }
[00:28:47] ...   |
[00:28:47] 167 | |         int_base! { Binary   for $Uint as $Uint -> Binary }
[00:28:47] ...   |
[00:28:47] 172 | |     }
[00:28:47] 173 | | }
[00:28:47]     | |_- in this expansion of `integer!`
[00:28:47]     | |_- in this expansion of `integer!`
[00:28:47] 174 |   integer! { isize, usize }
[00:28:47]     |   ------------------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/fmt/num.rs:134:35
[00:28:47] 130 | / macro_rules! int_base {
[00:28:47] 130 | / macro_rules! int_base {
[00:28:47] 131 | |     ($Trait:ident for $T:ident as $U:ident -> $Radix:ident) => {
[00:28:47] 132 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 133 | |         impl fmt::$Trait for $T {
[00:28:47] 134 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47] ...   |
[00:28:47] 138 | |     }
[00:28:47] 139 | | }
[00:28:47]     | |_- in this expansion of `int_base!`
[00:28:47]     | |_- in this expansion of `int_base!`
[00:28:47] ...
[00:28:47] 159 | / macro_rules! integer {
[00:28:47] 160 | |     ($Int:ident, $Uint:ident) => {
[00:28:47] 161 | |         int_base! { Binary   for $Int as $Uint  -> Binary }
[00:28:47] 162 | |         int_base! { Octal    for $Int as $Uint  -> Octal }
[00:28:47] ...   |
[00:28:47] 168 | |         int_base! { Octal    for $Uint as $Uint -> Octal }
[00:28:47] ...   |
[00:28:47] 172 | |     }
[00:28:47] 173 | | }
[00:28:47]     | |_- in this expansion of `integer!`
[00:28:47]     | |_- in this expansion of `integer!`
[00:28:47] 174 |   integer! { isize, usize }
[00:28:47]     |   ------------------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/fmt/num.rs:134:35
[00:28:47] 130 | / macro_rules! int_base {
[00:28:47] 130 | / macro_rules! int_base {
[00:28:47] 131 | |     ($Trait:ident for $T:ident as $U:ident -> $Radix:ident) => {
[00:28:47] 132 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 133 | |         impl fmt::$Trait for $T {
[00:28:47] 134 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47] ...   |
[00:28:47] 138 | |     }
[00:28:47] 139 | | }
[00:28:47]     | |_- in this expansion of `int_base!`
[00:28:47]     | |_- in this expansion of `int_base!`
[00:28:47] ...
[00:28:47] 159 | / macro_rules! integer {
[00:28:47] 160 | |     ($Int:ident, $Uint:ident) => {
[00:28:47] 161 | |         int_base! { Binary   for $Int as $Uint  -> Binary }
[00:28:47] 162 | |         int_base! { Octal    for $Int as $Uint  -> Octal }
[00:28:47] ...   |
[00:28:47] 169 | |         int_base! { LowerHex for $Uint as $Uint -> LowerHex }
[00:28:47] ...   |
[00:28:47] 172 | |     }
[00:28:47] 173 | | }
[00:28:47]     | |_- in this expansion of `integer!`
[00:28:47]     | |_- in this expansion of `integer!`
[00:28:47] 174 |   integer! { isize, usize }
[00:28:47]     |   ------------------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/fmt/num.rs:134:35
[00:28:47] 130 | / macro_rules! int_base {
[00:28:47] 130 | / macro_rules! int_base {
[00:28:47] 131 | |     ($Trait:ident for $T:ident as $U:ident -> $Radix:ident) => {
[00:28:47] 132 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 133 | |         impl fmt::$Trait for $T {
[00:28:47] 134 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47] ...   |
[00:28:47] 138 | |     }
[00:28:47] 139 | | }
[00:28:47]     | |_- in this expansion of `int_base!`
[00:28:47]     | |_- in this expansion of `int_base!`
[00:28:47] ...
[00:28:47] 159 | / macro_rules! integer {
[00:28:47] 160 | |     ($Int:ident, $Uint:ident) => {
[00:28:47] 161 | |         int_base! { Binary   for $Int as $Uint  -> Binary }
[00:28:47] 162 | |         int_base! { Octal    for $Int as $Uint  -> Octal }
[00:28:47] ...   |
[00:28:47] 170 | |         int_base! { UpperHex for $Uint as $Uint -> UpperHex }
[00:28:47]     | |         ----------------------------------------------------- in this macro invocation
[00:28:47] 171 | |         debug! { $Uint }
[00:28:47] 173 | | }
[00:28:47]     | |_- in this expansion of `integer!`
[00:28:47]     | |_- in this expansion of `integer!`
[00:28:47] 174 |   integer! { isize, usize }
[00:28:47]     |   ------------------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/fmt/num.rs:146:35
[00:28:47] 141 | / macro_rules! debug {
[00:28:47] 142 | |     ($T:ident) => {
[00:28:47] 143 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 143 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 144 | |         impl fmt::Debug for $T {
[00:28:47] 145 | |             #[inline]
[00:28:47] 146 | |             fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
[00:28:47] ...   |
[00:28:47] 156 | |     }
[00:28:47] 157 | | }
[00:28:47]     | |_- in this expansion of `debug!`
[00:28:47]     | |_- in this expansion of `debug!`
[00:28:47] 158 | 
[00:28:47] 159 | / macro_rules! integer {
[00:28:47] 160 | |     ($Int:ident, $Uint:ident) => {
[00:28:47] 161 | |         int_base! { Binary   for $Int as $Uint  -> Binary }
[00:28:47] 162 | |         int_base! { Octal    for $Int as $Uint  -> Octal }
[00:28:47] ...   |
---
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]    --> src/libcore/fmt/mod.rs:997:44
[00:28:47]     |
[00:28:47] 997 | pub fn write(output: &mut dyn Write, args: Arguments) -> Result {
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/fmt/mod.rs:1186:33
[00:28:47]      |
[00:28:47]      |
[00:28:47] 1186 |         fn write_prefix(f: &mut Formatter, sign: Option<char>, prefix: Option<&str>) -> Result {
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/fmt/mod.rs:1334:51
[00:28:47]      |
[00:28:47]      |
[00:28:47] 1334 |     fn pad_formatted_parts(&mut self, formatted: &flt2dec::Formatted) -> Result {
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/fmt/mod.rs:1373:53
[00:28:47]      |
[00:28:47]      |
[00:28:47] 1373 |     fn write_formatted_parts(&mut self, formatted: &flt2dec::Formatted) -> Result {
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/fmt/mod.rs:1456:38
[00:28:47]      |
[00:28:47]      |
[00:28:47] 1456 |     pub fn write_fmt(&mut self, fmt: Arguments) -> Result {
[00:28:47]      |                                      ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/fmt/mod.rs:1895:35
[00:28:47]      |
[00:28:47] 1895 |     fn write_fmt(&mut self, args: Arguments) -> Result {
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/fmt/mod.rs:1902:27
[00:28:47]      |
[00:28:47]      |
[00:28:47] 1902 |     fn fmt(&self, f: &mut Formatter) -> Result {
[00:28:47]      |                           ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/fmt/mod.rs:1914:35
[00:28:47]      |
[00:28:47] 1909 | / macro_rules! fmt_refs {
[00:28:47] 1910 | |     ($($tr:ident),*) => {
[00:28:47] 1912 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 1912 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 1913 | |         impl<T: ?Sized + $tr> $tr for &T {
[00:28:47] 1914 | |             fn fmt(&self, f: &mut Formatter) -> Result { $tr::fmt(&**self, f) }
[00:28:47] ...    |
[00:28:47] 1921 | |     }
[00:28:47] 1922 | | }
[00:28:47] 1922 | | }
[00:28:47]      | |_- in this expansion of `fmt_refs!`
[00:28:47] 1923 | 
[00:28:47] 1924 |   fmt_refs! { Debug, Display, Octal, Binary, LowerHex, UpperHex, LowerExp, UpperExp }
[00:28:47]      |   ----------------------------------------------------------------------------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/fmt/mod.rs:1918:35
[00:28:47]      |
[00:28:47]      |
[00:28:47] 1909 | / macro_rules! fmt_refs {
[00:28:47] 1910 | |     ($($tr:ident),*) => {
[00:28:47] 1912 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] ...    |
[00:28:47] ...    |
[00:28:47] 1918 | |             fn fmt(&self, f: &mut Formatter) -> Result { $tr::fmt(&**self, f) }
[00:28:47] ...    |
[00:28:47] 1921 | |     }
[00:28:47] 1922 | | }
[00:28:47] 1922 | | }
[00:28:47]      | |_- in this expansion of `fmt_refs!`
[00:28:47] 1923 | 
[00:28:47] 1924 |   fmt_refs! { Debug, Display, Octal, Binary, LowerHex, UpperHex, LowerExp, UpperExp }
[00:28:47]      |   ----------------------------------------------------------------------------------- in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/fmt/mod.rs:1928:27
[00:28:47]      |
[00:28:47] 1928 |     fn fmt(&self, _: &mut Formatter) -> Result {
---
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/fmt/mod.rs:2058:27
[00:28:47]      |
[00:28:47] 2058 |     fn fmt(&self, f: &mut Formatter) -> Result { Pointer::fmt(self, f) }
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/fmt/mod.rs:2062:27
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2062 |     fn fmt(&self, f: &mut Formatter) -> Result { Pointer::fmt(self, f) }
[00:28:47] 
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/fmt/mod.rs:2075:35
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2069 | / macro_rules! tuple {
[00:28:47] 2070 | |     () => ();
[00:28:47] 2071 | |     ( $($name:ident,)+ ) => (
[00:28:47] 2072 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 2075 | |             fn fmt(&self, f: &mut Formatter) -> Result {
[00:28:47]      | |                                   ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] 2086 | |     )
[00:28:47] 2086 | |     )
[00:28:47] 2087 | | }
[00:28:47]      | |_- in this expansion of `tuple!`
[00:28:47] ...
[00:28:47] 2094 |   tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }
[00:28:47]      |   ------------------------------------------------------------ in this macro invocation
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/fmt/mod.rs:2075:35
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2065 | / macro_rules! peel {
[00:28:47] 2066 | |     ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
[00:28:47] 2067 | | }
[00:28:47] 2067 | | }
[00:28:47]      | |_- in this expansion of `peel!` (#2)
[00:28:47] 2069 |   macro_rules! tuple {
[00:28:47]      |  _-
[00:28:47]      | |_|
[00:28:47]      | |
[00:28:47]      | |
[00:28:47] 2070 | |     () => ();
[00:28:47] 2071 | |     ( $($name:ident,)+ ) => (
[00:28:47] 2072 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 2075 | |             fn fmt(&self, f: &mut Formatter) -> Result {
[00:28:47]      | |                                   ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] ...    |
[00:28:47] 2085 | |         peel! { $($name,)* }
[00:28:47] 2086 | |     )
[00:28:47] 2087 | | }
[00:28:47]      | | -
[00:28:47]      | |_|
[00:28:47]      | |_|
[00:28:47]      | |_in this expansion of `tuple!` (#1)
[00:28:47]      |   in this expansion of `tuple!` (#3)
[00:28:47] ...
[00:28:47] 2094 | | tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }
[00:28:47]      | | ------------------------------------------------------------ in this macro invocation (#1)
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/fmt/mod.rs:2075:35
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2065 | / macro_rules! peel {
[00:28:47] 2066 | |     ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
[00:28:47] 2067 | | }
[00:28:47] 2067 | | }
[00:28:47]      | |_- in this expansion of `peel!` (#2)
[00:28:47] 2069 |   macro_rules! tuple {
[00:28:47]      |  _-
[00:28:47]      | |_|
[00:28:47]      | |
[00:28:47]      | |
[00:28:47] 2070 | |     () => ();
[00:28:47] 2071 | |     ( $($name:ident,)+ ) => (
[00:28:47] 2072 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 2075 | |             fn fmt(&self, f: &mut Formatter) -> Result {
[00:28:47]      | |                                   ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] ...    |
[00:28:47] 2085 | |         peel! { $($name,)* }
[00:28:47] 2086 | |     )
[00:28:47] 2087 | | }
[00:28:47]      | | -
[00:28:47]      | |_|
[00:28:47]      | |_|
[00:28:47]      | |_in this expansion of `tuple!` (#1)
[00:28:47]      |   in this expansion of `tuple!` (#3)
[00:28:47] ...
[00:28:47] 2094 | | tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }
[00:28:47]      | | ------------------------------------------------------------ in this macro invocation (#1)
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/fmt/mod.rs:2075:35
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2065 | / macro_rules! peel {
[00:28:47] 2066 | |     ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
[00:28:47] 2067 | | }
[00:28:47] 2067 | | }
[00:28:47]      | |_- in this expansion of `peel!` (#2)
[00:28:47] 2069 |   macro_rules! tuple {
[00:28:47]      |  _-
[00:28:47]      | |_|
[00:28:47]      | |
[00:28:47]      | |
[00:28:47] 2070 | |     () => ();
[00:28:47] 2071 | |     ( $($name:ident,)+ ) => (
[00:28:47] 2072 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 2075 | |             fn fmt(&self, f: &mut Formatter) -> Result {
[00:28:47]      | |                                   ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] ...    |
[00:28:47] 2085 | |         peel! { $($name,)* }
[00:28:47] 2086 | |     )
[00:28:47] 2087 | | }
[00:28:47]      | | -
[00:28:47]      | |_|
[00:28:47]      | |_|
[00:28:47]      | |_in this expansion of `tuple!` (#1)
[00:28:47]      |   in this expansion of `tuple!` (#3)
[00:28:47] ...
[00:28:47] 2094 | | tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }
[00:28:47]      | | ------------------------------------------------------------ in this macro invocation (#1)
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/fmt/mod.rs:2075:35
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2065 | / macro_rules! peel {
[00:28:47] 2066 | |     ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
[00:28:47] 2067 | | }
[00:28:47] 2067 | | }
[00:28:47]      | |_- in this expansion of `peel!` (#2)
[00:28:47] 2069 |   macro_rules! tuple {
[00:28:47]      |  _-
[00:28:47]      | |_|
[00:28:47]      | |
[00:28:47]      | |
[00:28:47] 2070 | |     () => ();
[00:28:47] 2071 | |     ( $($name:ident,)+ ) => (
[00:28:47] 2072 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 2075 | |             fn fmt(&self, f: &mut Formatter) -> Result {
[00:28:47]      | |                                   ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] ...    |
[00:28:47] 2085 | |         peel! { $($name,)* }
[00:28:47] 2086 | |     )
[00:28:47] 2087 | | }
[00:28:47]      | | -
[00:28:47]      | |_|
[00:28:47]      | |_|
[00:28:47]      | |_in this expansion of `tuple!` (#1)
[00:28:47]      |   in this expansion of `tuple!` (#3)
[00:28:47] ...
[00:28:47] 2094 | | tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }
[00:28:47]      | | ------------------------------------------------------------ in this macro invocation (#1)
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/fmt/mod.rs:2075:35
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2065 | / macro_rules! peel {
[00:28:47] 2066 | |     ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
[00:28:47] 2067 | | }
[00:28:47] 2067 | | }
[00:28:47]      | |_- in this expansion of `peel!` (#2)
[00:28:47] 2069 |   macro_rules! tuple {
[00:28:47]      |  _-
[00:28:47]      | |_|
[00:28:47]      | |
[00:28:47]      | |
[00:28:47] 2070 | |     () => ();
[00:28:47] 2071 | |     ( $($name:ident,)+ ) => (
[00:28:47] 2072 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 2075 | |             fn fmt(&self, f: &mut Formatter) -> Result {
[00:28:47]      | |                                   ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] ...    |
[00:28:47] 2085 | |         peel! { $($name,)* }
[00:28:47] 2086 | |     )
[00:28:47] 2087 | | }
[00:28:47]      | | -
[00:28:47]      | |_|
[00:28:47]      | |_|
[00:28:47]      | |_in this expansion of `tuple!` (#1)
[00:28:47]      |   in this expansion of `tuple!` (#3)
[00:28:47] ...
[00:28:47] 2094 | | tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }
[00:28:47]      | | ------------------------------------------------------------ in this macro invocation (#1)
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/fmt/mod.rs:2075:35
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2065 | / macro_rules! peel {
[00:28:47] 2066 | |     ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
[00:28:47] 2067 | | }
[00:28:47] 2067 | | }
[00:28:47]      | |_- in this expansion of `peel!` (#2)
[00:28:47] 2069 |   macro_rules! tuple {
[00:28:47]      |  _-
[00:28:47]      | |_|
[00:28:47]      | |
[00:28:47]      | |
[00:28:47] 2070 | |     () => ();
[00:28:47] 2071 | |     ( $($name:ident,)+ ) => (
[00:28:47] 2072 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 2075 | |             fn fmt(&self, f: &mut Formatter) -> Result {
[00:28:47]      | |                                   ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] ...    |
[00:28:47] 2085 | |         peel! { $($name,)* }
[00:28:47] 2086 | |     )
[00:28:47] 2087 | | }
[00:28:47]      | | -
[00:28:47]      | |_|
[00:28:47]      | |_|
[00:28:47]      | |_in this expansion of `tuple!` (#1)
[00:28:47]      |   in this expansion of `tuple!` (#3)
[00:28:47] ...
[00:28:47] 2094 | | tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }
[00:28:47]      | | ------------------------------------------------------------ in this macro invocation (#1)
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/fmt/mod.rs:2075:35
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2065 | / macro_rules! peel {
[00:28:47] 2066 | |     ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
[00:28:47] 2067 | | }
[00:28:47] 2067 | | }
[00:28:47]      | |_- in this expansion of `peel!` (#2)
[00:28:47] 2069 |   macro_rules! tuple {
[00:28:47]      |  _-
[00:28:47]      | |_|
[00:28:47]      | |
[00:28:47]      | |
[00:28:47] 2070 | |     () => ();
[00:28:47] 2071 | |     ( $($name:ident,)+ ) => (
[00:28:47] 2072 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 2075 | |             fn fmt(&self, f: &mut Formatter) -> Result {
[00:28:47]      | |                                   ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] ...    |
[00:28:47] 2085 | |         peel! { $($name,)* }
[00:28:47] 2086 | |     )
[00:28:47] 2087 | | }
[00:28:47]      | | -
[00:28:47]      | |_|
[00:28:47]      | |_|
[00:28:47]      | |_in this expansion of `tuple!` (#1)
[00:28:47]      |   in this expansion of `tuple!` (#3)
[00:28:47] ...
[00:28:47] 2094 | | tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }
[00:28:47]      | | ------------------------------------------------------------ in this macro invocation (#1)
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/fmt/mod.rs:2075:35
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2065 | / macro_rules! peel {
[00:28:47] 2066 | |     ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
[00:28:47] 2067 | | }
[00:28:47] 2067 | | }
[00:28:47]      | |_- in this expansion of `peel!` (#2)
[00:28:47] 2069 |   macro_rules! tuple {
[00:28:47]      |  _-
[00:28:47]      | |_|
[00:28:47]      | |
[00:28:47]      | |
[00:28:47] 2070 | |     () => ();
[00:28:47] 2071 | |     ( $($name:ident,)+ ) => (
[00:28:47] 2072 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 2075 | |             fn fmt(&self, f: &mut Formatter) -> Result {
[00:28:47]      | |                                   ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] ...    |
[00:28:47] 2085 | |         peel! { $($name,)* }
[00:28:47] 2086 | |     )
[00:28:47] 2087 | | }
[00:28:47]      | | -
[00:28:47]      | |_|
[00:28:47]      | |_|
[00:28:47]      | |_in this expansion of `tuple!` (#1)
[00:28:47]      |   in this expansion of `tuple!` (#3)
[00:28:47] ...
[00:28:47] 2094 | | tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }
[00:28:47]      | | ------------------------------------------------------------ in this macro invocation (#1)
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/fmt/mod.rs:2075:35
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2065 | / macro_rules! peel {
[00:28:47] 2066 | |     ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
[00:28:47] 2067 | | }
[00:28:47] 2067 | | }
[00:28:47]      | |_- in this expansion of `peel!` (#2)
[00:28:47] 2069 |   macro_rules! tuple {
[00:28:47]      |  _-
[00:28:47]      | |_|
[00:28:47]      | |
[00:28:47]      | |
[00:28:47] 2070 | |     () => ();
[00:28:47] 2071 | |     ( $($name:ident,)+ ) => (
[00:28:47] 2072 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 2075 | |             fn fmt(&self, f: &mut Formatter) -> Result {
[00:28:47]      | |                                   ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] ...    |
[00:28:47] 2085 | |         peel! { $($name,)* }
[00:28:47] 2086 | |     )
[00:28:47] 2087 | | }
[00:28:47]      | | -
[00:28:47]      | |_|
[00:28:47]      | |_|
[00:28:47]      | |_in this expansion of `tuple!` (#1)
[00:28:47]      |   in this expansion of `tuple!` (#3)
[00:28:47] ...
[00:28:47] 2094 | | tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }
[00:28:47]      | | ------------------------------------------------------------ in this macro invocation (#1)
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/fmt/mod.rs:2075:35
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2065 | / macro_rules! peel {
[00:28:47] 2066 | |     ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
[00:28:47] 2067 | | }
[00:28:47] 2067 | | }
[00:28:47]      | |_- in this expansion of `peel!` (#2)
[00:28:47] 2069 |   macro_rules! tuple {
[00:28:47]      |  _-
[00:28:47]      | |_|
[00:28:47]      | |
[00:28:47]      | |
[00:28:47] 2070 | |     () => ();
[00:28:47] 2071 | |     ( $($name:ident,)+ ) => (
[00:28:47] 2072 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 2075 | |             fn fmt(&self, f: &mut Formatter) -> Result {
[00:28:47]      | |                                   ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] ...    |
[00:28:47] 2085 | |         peel! { $($name,)* }
[00:28:47] 2086 | |     )
[00:28:47] 2087 | | }
[00:28:47]      | | -
[00:28:47]      | |_|
[00:28:47]      | |_|
[00:28:47]      | |_in this expansion of `tuple!` (#1)
[00:28:47]      |   in this expansion of `tuple!` (#3)
[00:28:47] ...
[00:28:47] 2094 | | tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }
[00:28:47]      | | ------------------------------------------------------------ in this macro invocation (#1)
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/fmt/mod.rs:2075:35
[00:28:47]      |
[00:28:47]      |
[00:28:47] 2065 | / macro_rules! peel {
[00:28:47] 2066 | |     ($name:ident, $($other:ident,)*) => (tuple! { $($other,)* })
[00:28:47] 2067 | | }
[00:28:47] 2067 | | }
[00:28:47]      | |_- in this expansion of `peel!` (#2)
[00:28:47] 2069 |   macro_rules! tuple {
[00:28:47]      |  _-
[00:28:47]      | |_|
[00:28:47]      | |
[00:28:47]      | |
[00:28:47] 2070 | |     () => ();
[00:28:47] 2071 | |     ( $($name:ident,)+ ) => (
[00:28:47] 2072 | |         #[stable(feature = "rust1", since = "1.0.0")]
[00:28:47] 2075 | |             fn fmt(&self, f: &mut Formatter) -> Result {
[00:28:47]      | |                                   ^^^^^^^^^- help: indicate the anonymous lifetime: `<'_>`
[00:28:47] ...    |
[00:28:47] ...    |
[00:28:47] 2085 | |         peel! { $($name,)* }
[00:28:47] 2086 | |     )
[00:28:47] 2087 | | }
[00:28:47]      | | -
[00:28:47]      | |_|
[00:28:47]      | |_|
[00:28:47]      | |_in this expansion of `tuple!` (#1)
[00:28:47]      |   in this expansion of `tuple!` (#3)
[00:28:47] ...
[00:28:47] 2094 | | tuple! { T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, }
[00:28:47]      | | ------------------------------------------------------------ in this macro invocation (#1)
[00:28:47] error: hidden lifetime parameters in types are deprecated
[00:28:47]     --> src/libcore/fmt/mod.rs:2098:27
[00:28:47]      |
[00:28:47] 2098 |     fn fmt(&self, f: &mut Formatter) -> Result {
---
travis_time:end:24a38310:start=1553772168746944745,finish=1553772168752964220,duration=6019475
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:167a66f0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0ad9be7a
travis_time:start:0ad9be7a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:040cce27
$ dmesg | grep -i kill
