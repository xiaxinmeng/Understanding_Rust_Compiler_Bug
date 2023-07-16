plain
[01:24:28] 
[01:24:34] error[E0412]: cannot find type `EvalResult` in this scope
[01:24:34]   --> src/tools/miri/src/fn_call.rs:17:10
[01:24:34]    |
[01:24:34] 17 |     ) -> EvalResult<'tcx, Option<&'mir mir::Mir<'tcx>>> {
[01:24:34] help: possible candidate is found in another module, you can import it into scope
[01:24:34]    |
[01:24:34] 1  | use rustc::middle::stability::EvalResult;
[01:24:34]    |
---
[01:24:34] 
[01:24:34] error[E0412]: cannot find type `EvalResult` in this scope
[01:24:34]    --> src/tools/miri/src/fn_call.rs:759:62
[01:24:34]     |
[01:24:34] 759 |     fn write_null(&mut self, dest: PlaceTy<'tcx, Borrow>) -> EvalResult<'tcx> {
[01:24:34] help: possible candidate is found in another module, you can import it into scope
[01:24:34]     |
[01:24:34] 1   | use rustc::middle::stability::EvalResult;
[01:24:34]     |
---
[01:24:34] 
[01:24:34] error[E0412]: cannot find type `EvalResult` in this scope
[01:24:34]   --> src/tools/miri/src/helpers.rs:12:46
[01:24:34]    |
[01:24:34] 12 |     fn resolve_path(&self, path: &[&str]) -> EvalResult<'tcx, ty::Instance<'tcx>> {
[01:24:34] help: possible candidate is found in another module, you can import it into scope
[01:24:34]    |
[01:24:34] 1  | use rustc::middle::stability::EvalResult;
[01:24:34]    |
[01:24:34]    |
[01:24:34] 
[01:24:34] error[E0412]: cannot find type `EvalResult` in this scope
[01:24:34]   --> src/tools/miri/src/helpers.rs:52:64
[01:24:34]    |
[01:24:34] 52 |         mut action: impl FnMut(Pointer<Borrow>, Size, bool) -> EvalResult<'tcx>,
[01:24:34] help: possible candidate is found in another module, you can import it into scope
[01:24:34]    |
[01:24:34] 1  | use rustc::middle::stability::EvalResult;
[01:24:34]    |
---
[01:24:34] 
[01:24:34] error[E0412]: cannot find type `EvalResult` in this scope
[01:24:34]    --> src/tools/miri/src/helpers.rs:123:55
[01:24:34]     |
[01:24:34] 123 |             where F: FnMut(MPlaceTy<'tcx, Borrow>) -> EvalResult<'tcx>
[01:24:34] help: possible candidate is found in another module, you can import it into scope
[01:24:34]     |
[01:24:34] 1   | use rustc::middle::stability::EvalResult;
[01:24:34]     |
[01:24:34]     |
[01:24:34] 
[01:24:34] error[E0412]: cannot find type `EvalResult` in this scope
[01:24:34]    --> src/tools/miri/src/helpers.rs:134:49
[01:24:34]     |
[01:24:34] 134 |             F: FnMut(MPlaceTy<'tcx, Borrow>) -> EvalResult<'tcx>
[01:24:34] help: possible candidate is found in another module, you can import it into scope
[01:24:34]     |
[01:24:34] 1   | use rustc::middle::stability::EvalResult;
[01:24:34]     |
[01:24:34]     |
[01:24:34] 
[01:24:34] error[E0412]: cannot find type `EvalResult` in this scope
[01:24:34]    --> src/tools/miri/src/helpers.rs:144:69
[01:24:34]     |
[01:24:34] 144 |             fn visit_value(&mut self, v: MPlaceTy<'tcx, Borrow>) -> EvalResult<'tcx>
[01:24:34] help: possible candidate is found in another module, you can import it into scope
[01:24:34]     |
[01:24:34] 1   | use rustc::middle::stability::EvalResult;
[01:24:34]     |
[01:24:34]     |
[01:24:34] 
[01:24:34] error[E0412]: cannot find type `EvalResult` in this scope
[01:24:34]    --> src/tools/miri/src/helpers.rs:167:44
[01:24:34]     |
[01:24:34] 167 |                 fields: impl Iterator<Item=EvalResult<'tcx, MPlaceTy<'tcx, Borrow>>>,
[01:24:34] help: possible candidate is found in another module, you can import it into scope
[01:24:34]     |
[01:24:34] 1   | use rustc::middle::stability::EvalResult;
[01:24:34]     |
---
[01:24:34] 
[01:24:34] error[E0412]: cannot find type `EvalResult` in this scope
[01:24:34]    --> src/tools/miri/src/helpers.rs:177:59
[01:24:34]     |
[01:24:34] 177 |                         let mut places = fields.collect::<EvalResult<'tcx, Vec<MPlaceTy<'tcx, Borrow>>>>()?;
[01:24:34] help: possible candidate is found in another module, you can import it into scope
[01:24:34]     |
[01:24:34] 1   | use rustc::middle::stability::EvalResult;
[01:24:34]     |
[01:24:34]     |
[01:24:34] 
[01:24:34] error[E0412]: cannot find type `EvalResult` in this scope
[01:24:34]    --> src/tools/miri/src/helpers.rs:189:69
[01:24:34]     |
[01:24:34] 189 |             fn visit_union(&mut self, v: MPlaceTy<'tcx, Borrow>) -> EvalResult<'tcx>
[01:24:34] help: possible candidate is found in another module, you can import it into scope
[01:24:34]     |
[01:24:34] 1   | use rustc::middle::stability::EvalResult;
[01:24:34]     |
[01:24:34]     |
[01:24:34] 
[01:24:34] error[E0412]: cannot find type `EvalResult` in this scope
[01:24:34]    --> src/tools/miri/src/helpers.rs:203:74
[01:24:34]     |
[01:24:34] 203 |             fn visit_primitive(&mut self, _v: MPlaceTy<'tcx, Borrow>) -> EvalResult<'tcx>
[01:24:34] help: possible candidate is found in another module, you can import it into scope
[01:24:34]     |
[01:24:34] 1   | use rustc::middle::stability::EvalResult;
[01:24:34]     |
---
[01:24:34] 
[01:24:34] error[E0412]: cannot find type `EvalResult` in this scope
[01:24:34]    --> src/tools/miri/src/lib.rs:391:10
[01:24:34]     |
[01:24:34] 391 |     ) -> EvalResult<'tcx, Option<&'mir mir::Mir<'tcx>>> {
[01:24:34] help: possible candidate is found in another module, you can import it into scope
[01:24:34]     |
[01:24:34] 24  | use rustc::middle::stability::EvalResult;
[01:24:34]     |
---
travis_time:end:232453d8:start=1554456564515610212,finish=1554456564521742200,duration=6131988
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:020ee609
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0064d250
travis_time:start:0064d250
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:25dfdb8e
$ dmesg | grep -i kill
