plain
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:45] 
[01:01:45] running 75 tests
[01:01:48] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:488:22
[01:01:48] i...i..ii.F..i.............ii.....F...iii......i..i...i...ii..i..i..ii.....
[01:01:48] 
[01:01:48] ---- [codegen] codegen/align-struct.rs stdout ----
[01:01:48]  
[01:01:48]  
[01:01:48] error: verification with 'FileCheck' failed
[01:01:48] status: exit code: 1
[01:01:48] command: "/usr/lib/llvm-3.9/bin/FileCheck" "--input-file" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/align-struct.ll" "/checkout/src/test/codegen/align-struct.rs"
[01:01:48] ------------------------------------------
[01:01:48] 
[01:01:48] ------------------------------------------
[01:01:48] stderr:
[01:01:48] stderr:
[01:01:48] ------------------------------------------
[01:01:48] /checkout/src/test/codegen/align-struct.rs:32:11: error: expected string not found in input
[01:01:48] // CHECK: %Enum4 = type { [0 x i32], i32, [1 x i32] }
[01:01:48]           ^
[01:01:48] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/align-struct.ll:8:1: note: scanning from here
[01:01:48] %"Enum4::A" = type { [1 x i32], i32, [0 x i32] }
[01:01:48] ^
[01:01:48] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/align-struct.ll:9:1: note: possible intended match here
[01:01:48] %Enum64 = type { [0 x i32], i32, [31 x i32] }
[01:01:48] ^
[01:01:48] /checkout/src/test/codegen/align-struct.rs:62:11: error: expected string not found in input
[01:01:48] // CHECK: %e4 = alloca %Enum4, align 4
[01:01:48]           ^
[01:01:48] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/align-struct.ll:77:27: note: scanning from here
[01:01:48] define { i32, i32 } @enum4(i32 %a) unnamed_addr #0 {
[01:01:48]                           ^
[01:01:48] /checkout/obj/build/x86_64-unknown-linux-gnu/test/codegen/align-struct.ll:79:2: note: possible intended match here
[01:01:48]  %e4 = alloca { i32, i32 }, align 4
[01:01:48]  ^
[01:01:48] ------------------------------------------
[01:01:48] 
[01:01:48] thread '[codegen] codegen/align-struct.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2956:9
[01:01:48] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:01:48] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:01:48] 
[01:01:48] ---- [codegen] codegen/lifetime_start_end.rs stdout ----
[01:01:48]  
[01:01:48] error: verification with 'FileCheck' failed
[01:01:48] status: exit code: 1
[01:01:48] command: "/usr/lib/llvm-3.9/bin/FileCheck" "--iead https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed, 25 Apr 2018 12:46:55 GMT
travis_time:end:18c16774:start=1524660415382606767,finish=1524660415552996221,duration=170389454

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
