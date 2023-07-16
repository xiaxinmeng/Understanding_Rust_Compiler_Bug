plain
---- [ui] ui/abi/stack-probes-lto.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/stack-probes-lto.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/stack-probes-lto/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-C" "lto" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/stack-probes-lto/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
inlinable function call in a function with debug info must have a !dbg location
  tail call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !9609
inlinable function call in a function with debug info must have a !dbg location
  tail call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !9609
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !24812
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !22456
inlinable function call in a function with debug info must have a !dbg location
  tail call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !24922
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !24811
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !22455
inlinable function call in a function with debug info must have a !dbg location
  tail call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !24921
inlinable function call in a function with debug info must have a !dbg location
  tail call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !32505
inlinable function call in a function with debug info must have a !dbg location
  tail call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !32401
inlinable function call in a function with debug info must have a !dbg location
  tail call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !32504
inlinable function call in a function with debug info must have a !dbg location
  tail call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !32400
inlinable function call in a function with debug info must have a !dbg location
  tail call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14
inlinable function call in a function with debug info must have a !dbg location
  tail call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14
inlinable function call in a function with debug info must have a !dbg location
  tail call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !45683
inlinable function call in a function with debug info must have a !dbg location
  tail call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !45682
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !49147
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !50031
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !53477
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !49146
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !50030
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !53476
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14
inlinable function call in a function with debug info must have a !dbg location
  tail call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14
inlinable function call in a function with debug info must have a !dbg location
  tail call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !63595
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !63837
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !63448
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !63594
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !63836
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !63447
inlinable function call in a function with debug info must have a !dbg location
  tail call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !68645
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !69734
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !70355
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !72388
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !73094
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !73378
inlinable function call in a function with debug info must have a !dbg location
  tail call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !68644
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !69733
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !70354
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !72387
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !73093
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !73377
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !74422
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !74421
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !74726
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !74725
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !76197
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !76196
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !76326
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !76325
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !76409
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !76408
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !77485
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !77648
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !77484
inlinable function call in a function with debug info must have a !dbg location
  call fastcc void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #14, !noalias !77647
LLVM ERROR: Broken module found, compilation aborted!
------------------------------------------


---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#fat1 stdout ----
---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#fat1 stdout ----

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i586-unknown-linux-gnu
error in revision `fat1`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--cfg" "fat1" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.fat1/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-C" "opt-level=1" "-C" "lto=fat" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.fat1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !8221
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !8221
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !20406
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !20405
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !22970
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !20587
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !23084
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !26314
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !22969
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !20586
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !23083
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !26313
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !30469
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !30369
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !30468
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !30368
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !43548
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !43547
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !44151
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !44150
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !47170
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !47772
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !51209
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !47169
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !47771
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !51208
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !59416
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !59679
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !59415
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !59678
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !60206
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !60205
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !60353
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !60352
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !61488
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !61487
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !61627
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !61626
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !61665
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !61664
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !63079
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !63110
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !63078
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10, !noalias !63109
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #10
LLVM ERROR: Broken module found, compilation aborted!
------------------------------------------


---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#fat0 stdout ----
---- [ui] ui/extern/issue-64655-extern-rust-must-allow-unwind.rs#fat0 stdout ----

error in revision `fat0`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--cfg" "fat0" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.fat0/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-C" "opt-level=0" "-C" "lto=fat" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/issue-64655-extern-rust-must-allow-unwind.fat0/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !56558
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !56558
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !69866
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !69865
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !72416
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !70044
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !72529
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !75758
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !72415
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !70043
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !72528
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !75757
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !79911
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !79811
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !79910
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !79810
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !92982
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !92981
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !93585
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !93584
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !96602
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !97204
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !100641
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !96601
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !97203
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !100640
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !106240
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !106239
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !107909
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !107908
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !114081
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !114080
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !115951
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !115950
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !118326
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !118325
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !118990
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !118989
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !119538
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !119537
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !120044
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !120043
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !120297
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !120297
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !120296
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !120296
inlinable function call in a function with debug info must have a !dbg location
  call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !120661
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5, !noalias !120660
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
inlinable function call in a function with debug info must have a !dbg location
  tail call void @_ZN4core9panicking15panic_no_unwind17h1ecfed293e8d013dE() #5
