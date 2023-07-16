plain

running 339 tests
ii...i.........i....i..i.................iii........ii.i.......i.................ii..... 88/339
............i..............i................i...iii........i..i......i........i.......i. 176/339
...............i..Fi...i.....ii..i.ii.............iiii........................i.i.ii.i.. 264/339
Some tests failed in compiletest suite=codegen mode=codegen host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:


---- [codegen] src/test/codegen/noalias-rwlockreadguard.rs stdout ----

error: verification with 'FileCheck' failed
status: exit status: 1
command: "/usr/lib/llvm-12/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/noalias-rwlockreadguard/noalias-rwlockreadguard.ll" "/checkout/src/test/codegen/noalias-rwlockreadguard.rs" "--check-prefixes" "CHECK,NONMSVC"
stdout: none
--- stderr -------------------------------
/checkout/src/test/codegen/noalias-rwlockreadguard.rs:12:15: error: CHECK-NOT: excluded string found in input
// CHECK-NOT: noalias
              ^
/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/noalias-rwlockreadguard/noalias-rwlockreadguard.ll:293:70: note: found here
declare void @_ZN4core9panicking5panic17h1b3cb7cbf63f3b47E([0 x i8]* noalias noundef nonnull readonly align 1, i64, %"core::panic::location::Location"* noalias noundef readonly align 8 dereferenceable(24)) unnamed_addr #4


Input file: /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/noalias-rwlockreadguard/noalias-rwlockreadguard.ll
Check file: /checkout/src/test/codegen/noalias-rwlockreadguard.rs

-dump-input=help explains the following input dump.
Input was:
<<<<<<
        .
        .
        .
        .
      288: ; Function Attrs: argmemonly nofree nosync nounwind willreturn
      289: declare void @llvm.lifetime.end.p0i8(i64 immarg, i8* nocapture) #3
      291: ; core::panicking::panic
      291: ; core::panicking::panic
      292: ; Function Attrs: cold noinline noreturn nonlazybind uwtable
      293: declare void @_ZN4core9panicking5panic17h1b3cb7cbf63f3b47E([0 x i8]* noalias noundef nonnull readonly align 1, i64, %"core::panic::location::Location"* noalias noundef readonly align 8 dereferenceable(24)) unnamed_addr #4
not:12                                                                          !~~~~~~                                                                                                                                                  error: no match expected
      294: 
      295: ; std::sys::unix::locks::futex_rwlock::RwLock::wake_writer_or_readers
      296: ; Function Attrs: cold nonlazybind uwtable
      297: declare void @_ZN3std3sys4unix5locks12futex_rwlock6RwLock22wake_writer_or_readers17hfe00d6cdfff8143aE(%"std::sys::unix::locks::futex_rwlock::RwLock"* noundef align 4 dereferenceable(8), i32) unnamed_addr #5
        .
        .
        .
>>>>>>
>>>>>>
------------------------------------------



failures:
    [codegen] src/test/codegen/noalias-rwlockreadguard.rs
test result: FAILED. 282 passed; 1 failed; 56 ignored; 0 measured; 0 filtered out; finished in 3.29s

Build completed unsuccessfully in 0:11:06
