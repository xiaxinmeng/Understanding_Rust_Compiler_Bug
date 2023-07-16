plain
travis_time:end:029bbf80:start=1555888092248178090,finish=1555888176561016345,duration=84312838255
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:13:14]    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
[00:13:16] error[E0432]: unresolved import `super::InterpResult`
[00:13:16]   --> src/librustc_mir/interpret/place.rs:16:44
[00:13:16]    |
[00:13:16] 16 |     GlobalId, AllocId, Allocation, Scalar, InterpResult, Pointer, PointerArithmetic,
[00:13:16]    |                                            ^^^^^^^^^^^^ no `InterpResult` in `interpret`
[00:13:23] error[E0412]: cannot find type `EvalResult` in this scope
[00:13:23]    --> src/librustc_mir/interpret/place.rs:134:28
[00:13:23]     |
[00:13:23]     |
[00:13:23] 134 |     pub fn to_ptr(self) -> EvalResult<'tcx, Pointer<Tag>> {
[00:13:23] help: possible candidates are found in other modules, you can import them into scope
[00:13:23]     |
[00:13:23] 5   | use rustc::middle::stability::EvalResult;
[00:13:23]     |
---
[00:13:23] 
[00:13:23] error[E0412]: cannot find type `EvalResult` in this scope
[00:13:23]    --> src/librustc_mir/interpret/place.rs:207:57
[00:13:23]     |
[00:13:23] 207 |     pub(super) fn len(self, cx: &impl HasDataLayout) -> EvalResult<'tcx, u64> {
[00:13:23] help: possible candidates are found in other modules, you can import them into scope
[00:13:23]     |
[00:13:23] 5   | use rustc::middle::stability::EvalResult;
[00:13:23]     |
[00:13:23]     |
[00:13:23] 5   | use rustc::mir::interpret::EvalResult;
[00:13:23]     |
[00:13:23] 
[00:13:23] error[E0412]: cannot find type `EvalResult` in this scope
[00:13:23]    --> src/librustc_mir/interpret/place.rs:226:35
[00:13:23]     |
[00:13:23] 226 |     pub(super) fn vtable(self) -> EvalResult<'tcx, Pointer<Tag>> {
[00:13:23] help: possible candidates are found in other modules, you can import them into scope
[00:13:23]     |
[00:13:23] 5   | use rustc::middle::stability::EvalResult;
[00:13:23]     |
[00:13:23]     |
[00:13:23] 5   | use rustc::mir::interpret::EvalResult;
[00:13:23]     |
[00:13:23] 
[00:13:23] error[E0412]: cannot find type `EvalResult` in this scope
[00:13:23]    --> src/librustc_mir/interpret/place.rs:281:28
[00:13:23]     |
[00:13:23] 281 |     pub fn to_ptr(self) -> EvalResult<'tcx, Pointer<Tag>> {
[00:13:23] help: possible candidates are found in other modules, you can import them into scope
[00:13:23]     |
[00:13:23] 5   | use rustc::middle::stability::EvalResult;
[00:13:23]     |
[00:13:23]     |
[00:13:23] 5   | use rustc::mir::interpret::EvalResult;
[00:13:23]     |
[00:13:23] 
[00:13:23] error[E0412]: cannot find type `EvalResult` in this scope
[00:13:23]    --> src/librustc_mir/interpret/place.rs:310:10
[00:13:23]     |
[00:13:23] 310 |     ) -> EvalResult<'tcx, MPlaceTy<'tcx, M::PointerTag>> {
[00:13:23] help: possible candidates are found in other modules, you can import them into scope
[00:13:23]     |
[00:13:23] 5   | use rustc::middle::stability::EvalResult;
[00:13:23]     |
[00:13:23]     |
[00:13:23] 5   | use rustc::mir::interpret::EvalResult;
[00:13:23]     |
[00:13:23] 
[00:13:24] error[E0412]: cannot find type `EvalResult` in this scope
[00:13:24]    --> src/librustc_mir/interpret/place.rs:333:10
[00:13:24]     |
[00:13:24] 333 |     ) -> EvalResult<'tcx, MPlaceTy<'tcx, M::PointerTag>> {
[00:13:24] help: possible candidates are found in other modules, you can import them into scope
[00:13:24]     |
[00:13:24] 5   | use rustc::middle::stability::EvalResult;
[00:13:24]     |
[00:13:24]     |
[00:13:24] 5   | use rustc::mir::interpret::EvalResult;
[00:13:24]     |
[00:13:24] 
[00:13:24] error[E0412]: cannot find type `EvalResult` in this scope
[00:13:24]    --> src/librustc_mir/interpret/place.rs:357:10
[00:13:24]     |
[00:13:24] 357 |     ) -> EvalResult<'tcx, MPlaceTy<'tcx, M::PointerTag>> {
[00:13:24] help: possible candidates are found in other modules, you can import them into scope
[00:13:24]     |
[00:13:24] 5   | use rustc::middle::stability::EvalResult;
[00:13:24]     |
[00:13:24]     |
[00:13:24] 5   | use rustc::mir::interpret::EvalResult;
[00:13:24]     |
[00:13:24] 
[00:13:24] error[E0412]: cannot find type `EvalResult` in this scope
[00:13:24]    --> src/librustc_mir/interpret/place.rs:413:9
[00:13:24]     |
[00:13:24] 413 |         EvalResult<'tcx, impl Iterator<Item=EvalResult<'tcx, MPlaceTy<'tcx, Tag>>> + 'a>
[00:13:24] help: possible candidates are found in other modules, you can import them into scope
[00:13:24]     |
[00:13:24] 5   | use rustc::middle::stability::EvalResult;
[00:13:24]     |
[00:13:24]     |
[00:13:24] 5   | use rustc::mir::interpret::EvalResult;
[00:13:24]     |
[00:13:24] 
[00:13:24] error[E0412]: cannot find type `EvalResult` in this scope
[00:13:24]    --> src/librustc_mir/interpret/place.rs:413:45
[00:13:24]     |
[00:13:24] 413 |         EvalResult<'tcx, impl Iterator<Item=EvalResult<'tcx, MPlaceTy<'tcx, Tag>>> + 'a>
[00:13:24] help: possible candidates are found in other modules, you can import them into scope
[00:13:24]     |
[00:13:24] 5   | use rustc::middle::stability::EvalResult;
[00:13:24]     |
[00:13:24]     |
[00:13:24] 5   | use rustc::mir::interpret::EvalResult;
[00:13:24]     |
[00:13:24] 
[00:13:24] error[E0412]: cannot find type `EvalResult` in this scope
[00:13:24]    --> src/librustc_mir/interpret/place.rs:430:10
[00:13:24]     |
[00:13:24] 430 |     ) -> EvalResult<'tcx, MPlaceTy<'tcx, M::PointerTag>> {
[00:13:24] help: possible candidates are found in other modules, you can import them into scope
[00:13:24]     |
[00:13:24] 5   | use rustc::middle::stability::EvalResult;
[00:13:24]     |
[00:13:24]     |
[00:13:24] 5   | use rustc::mir::interpret::EvalResult;
[00:13:24]     |
[00:13:24] 
[00:13:24] error[E0412]: cannot find type `EvalResult` in this scope
[00:13:24]    --> src/librustc_mir/interpret/place.rs:464:10
[00:13:24]     |
[00:13:24] 464 |     ) -> EvalResult<'tcx, MPlaceTy<'tcx, M::PointerTag>> {
[00:13:24] help: possible candidates are found in other modules, you can import them into scope
[00:13:24]     |
[00:13:24] 5   | use rustc::middle::stability::EvalResult;
[00:13:24]     |
[00:13:24]     |
[00:13:24] 5   | use rustc::mir::interpret::EvalResult;
[00:13:24]     |
[00:13:24] 
[00:13:24] error[E0412]: cannot find type `EvalResult` in this scope
[00:13:24]    --> src/librustc_mir/interpret/place.rs:475:10
[00:13:24]     |
[00:13:24] 475 |     ) -> EvalResult<'tcx, MPlaceTy<'tcx, M::PointerTag>> {
[00:13:24] help: possible candidates are found in other modules, you can import them into scope
[00:13:24]     |
[00:13:24] 5   | use rustc::middle::stability::EvalResult;
[00:13:24]     |
[00:13:24]     |
[00:13:24] 5   | use rustc::mir::interpret::EvalResult;
[00:13:24]     |
[00:13:24] 
[00:13:24] error[E0412]: cannot find type `EvalResult` in this scope
[00:13:24]    --> src/librustc_mir/interpret/place.rs:520:10
[00:13:24]     |
[00:13:24] 520 |     ) -> EvalResult<'tcx, PlaceTy<'tcx, M::PointerTag>> {
[00:13:24] help: possible candidates are found in other modules, you can import them into scope
[00:13:24]     |
[00:13:24] 5   | use rustc::middle::stability::EvalResult;
[00:13:24]     |
[00:13:24]     |
[00:13:24] 5   | use rustc::mir::interpret::EvalResult;
[00:13:24]     |
[00:13:24] 
[00:13:24] error[E0412]: cannot find type `EvalResult` in this scope
[00:13:24]    --> src/librustc_mir/interpret/place.rs:531:10
[00:13:24]     |
[00:13:24] 531 |     ) -> EvalResult<'tcx, PlaceTy<'tcx, M::PointerTag>> {
[00:13:24] help: possible candidates are found in other modules, you can import them into scope
[00:13:24]     |
[00:13:24] 5   | use rustc::middle::stability::EvalResult;
[00:13:24]     |
[00:13:24]     |
[00:13:24] 5   | use rustc::mir::interpret::EvalResult;
[00:13:24]     |
[00:13:24] 
[00:13:24] error[E0412]: cannot find type `EvalResult` in this scope
[00:13:24]    --> src/librustc_mir/interpret/place.rs:568:10
[00:13:24]     |
[00:13:24] 568 |     ) -> EvalResult<'tcx, MPlaceTy<'tcx, M::PointerTag>> {
[00:13:24] help: possible candidates are found in other modules, you can import them into scope
[00:13:24]     |
[00:13:24] 5   | use rustc::middle::stability::EvalResult;
[00:13:24]     |
[00:13:24]     |
[00:13:24] 5   | use rustc::mir::interpret::EvalResult;
[00:13:24]     |
[00:13:24] 
[00:13:24] error[E0412]: cannot find type `EvalResult` in this scope
[00:13:24]    --> src/librustc_mir/interpret/place.rs:613:10
[00:13:24]     |
[00:13:24] 613 |     ) -> EvalResult<'tcx, PlaceTy<'tcx, M::PointerTag>> {
[00:13:24] help: possible candidates are found in other modules, you can import them into scope
[00:13:24]     |
[00:13:24] 5   | use rustc::middle::stability::EvalResult;
[00:13:24]     |
---
[00:13:24] 
[00:13:24] error[E0412]: cannot find type `EvalResult` in this scope
[00:13:24]    --> src/librustc_mir/interpret/place.rs:906:10
[00:13:24]     |
[00:13:24] 906 |     ) -> EvalResult<'tcx, (MPlaceTy<'tcx, M::PointerTag>, Option<Size>)> {
[00:13:24] help: possible candidates are found in other modules, you can import them into scope
[00:13:24]     |
[00:13:24] 5   | use rustc::middle::stability::EvalResult;
[00:13:24]     |
[00:13:24]     |
[00:13:24] 5   | use rustc::mir::interpret::EvalResult;
[00:13:24]     |
[00:13:24] 
[00:13:24] error[E0412]: cannot find type `EvalResult` in this scope
[00:13:24]    --> src/librustc_mir/interpret/place.rs:959:10
[00:13:24]     |
[00:13:24] 959 |     ) -> EvalResult<'tcx, MPlaceTy<'tcx, M::PointerTag>> {
[00:13:24] help: possible candidates are found in other modules, you can import them into scope
[00:13:24]     |
[00:13:24] 5   | use rustc::middle::stability::EvalResult;
[00:13:24]     |
---
[00:13:24] 
[00:13:24] error[E0412]: cannot find type `EvalResult` in this scope
[00:13:24]     --> src/librustc_mir/interpret/place.rs:1034:10
[00:13:24]      |
[00:13:24] 1034 |     ) -> EvalResult<'tcx, MPlaceTy<'tcx, M::PointerTag>> {
[00:13:24] help: possible candidates are found in other modules, you can import them into scope
[00:13:24]      |
[00:13:24] 5    | use rustc::middle::stability::EvalResult;
[00:13:24]      |
[00:13:24]      |
[00:13:24] 5    | use rustc::mir::interpret::EvalResult;
[00:13:24]      |
[00:13:24] 
[00:13:24] error[E0412]: cannot find type `EvalResult` in this scope
[00:13:24]     --> src/librustc_mir/interpret/place.rs:1047:8
[00:13:24]      |
[00:13:24] 1047 |     -> EvalResult<'tcx, (ty::Instance<'tcx>, MPlaceTy<'tcx, M::PointerTag>)> {
[00:13:24] help: possible candidates are found in other modules, you can import them into scope
[00:13:24]      |
[00:13:24] 5    | use rustc::middle::stability::EvalResult;
[00:13:24]      |
[00:13:24]      |
[00:13:24] 5    | use rustc::mir::interpret::EvalResult;
[00:13:24]      |
[00:13:24] 
[00:13:26] error[E0107]: wrong number of lifetime arguments: expected 0, found 1
[00:13:26]    --> src/librustc_mir/interpret/place.rs:547:41
[00:13:26]     |
[00:13:26] 547 |         proj_elem: &mir::ProjectionElem<'tcx, mir::Local, Ty<'tcx>>,
[00:13:26] 
[00:13:26] error: aborting due to 32 previous errors
[00:13:26] 
[00:13:26] Some errors occurred: E0107, E0412, E0432.
---
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:004fdf72
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access '/home/travis/Library/Logs/Dsuch file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:17642aea
$ dmesg | grep -i kill
