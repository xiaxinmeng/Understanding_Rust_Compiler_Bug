
failures:

---- [codegen] codegen/alloc-optimisation.rs stdout ----
	
error: verification with 'FileCheck' failed
status: exit code: 1
command: "/home/simon/rust1/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/home/simon/rust1/build/x86_64-unknown-linux-gnu/test/codegen/alloc-optimisation.ll" "/home/simon/rust1/src/test/codegen/alloc-optimisation.rs"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/home/simon/rust1/src/test/codegen/alloc-optimisation.rs:19:17: error: CHECK-NEXT: is not on the line after the previous match
 // CHECK-NEXT: ret void
                ^
/home/simon/rust1/build/x86_64-unknown-linux-gnu/test/codegen/alloc-optimisation.ll:52:2: note: 'next' match was here
 ret void
 ^
/home/simon/rust1/build/x86_64-unknown-linux-gnu/test/codegen/alloc-optimisation.ll:40:7: note: previous match ended here
start:
      ^
/home/simon/rust1/build/x86_64-unknown-linux-gnu/test/codegen/alloc-optimisation.ll:41:1: note: non-matching line after previous match is here
 %0 = tail call i8* @__rust_alloc(i64 4, i64 4) #6
^

------------------------------------------

thread '[codegen] codegen/alloc-optimisation.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.

---- [codegen] codegen/vec-optimizes-away.rs stdout ----
	
error: verification with 'FileCheck' failed
status: exit code: 1
command: "/home/simon/rust1/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--input-file" "/home/simon/rust1/build/x86_64-unknown-linux-gnu/test/codegen/vec-optimizes-away.ll" "/home/simon/rust1/src/test/codegen/vec-optimizes-away.rs"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/home/simon/rust1/src/test/codegen/vec-optimizes-away.rs:19:17: error: CHECK-NEXT: is not on the line after the previous match
 // CHECK-NEXT: ret i32 6
                ^
/home/simon/rust1/build/x86_64-unknown-linux-gnu/test/codegen/vec-optimizes-away.ll:52:2: note: 'next' match was here
 ret i32 6
 ^
/home/simon/rust1/build/x86_64-unknown-linux-gnu/test/codegen/vec-optimizes-away.ll:40:7: note: previous match ended here
start:
      ^
/home/simon/rust1/build/x86_64-unknown-linux-gnu/test/codegen/vec-optimizes-away.ll:41:1: note: non-matching line after previous match is here
 %0 = tail call i8* @__rust_alloc(i64 12, i64 4) #6
^

------------------------------------------

thread '[codegen] codegen/vec-optimizes-away.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
