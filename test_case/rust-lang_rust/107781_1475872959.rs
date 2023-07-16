
> The unwind destination does not have an exception handling instruction!
>   invoke void @_ZN5alloc7raw_vec17capacity_overflow17h250e7d3b3e97c20eE() #30
>           to label %2100 unwind label %2277
> The unwind destination does not have an exception handling instruction!
>   invoke void @_ZN5alloc5alloc18handle_alloc_error17h7750607dc7de1938E(i32 %2095, i32 noundef 1) #30
>           to label %2105 unwind label %2277
> The unwind destination does not have an exception handling instruction!
>   invoke void @_ZN5alloc5alloc18handle_alloc_error17h7750607dc7de1938E(i32 2, i32 noundef 1) #30
>           to label %2271 unwind label %2277
> The unwind destination does not have an exception handling instruction!
>   invoke void @_ZN5alloc5alloc18handle_alloc_error17h7750607dc7de1938E(i32 8, i32 noundef 4) #30
>           to label %2276 unwind label %2277
> CleanupPadInst not the first non-PHI instruction in the block.
>   %2282 = cleanuppad within none []
> The unwind destination does not have an exception handling instruction!
>   invoke fastcc void @"_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17hcee07cf79bbc96bcE.llvm.1999991953246885693"(ptr nonnull %107) #31 [ "funclet"(token %2306) ]
>           to label %2307 unwind label %2277
> CleanupReturnInst must unwind to an EH block which is not a landingpad.
>   cleanupret from %2306 unwind label %2277
> The unwind destination does not have an exception handling instruction!
>   invoke void @_ZN5alloc5alloc18handle_alloc_error17h7750607dc7de1938E(i32 9, i32 noundef 1) #30
>           to label %2314 unwind label %2277
> The unwind destination does not have an exception handling instruction!
>   invoke fastcc void @"_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17hcee07cf79bbc96bcE.llvm.1999991953246885693"(ptr nonnull %109) #31 [ "funclet"(token %2334) ]
>           to label %2335 unwind label %2277
> CleanupReturnInst must unwind to an EH block which is not a landingpad.
>   cleanupret from %2334 unwind label %2277
> CleanupReturnInst must unwind to an EH block which is not a landingpad.
>   cleanupret from %2340 unwind label %2277
> CleanupReturnInst must unwind to an EH block which is not a landingpad.
>   cleanupret from %2458 unwind label %2277
> CleanupReturnInst must unwind to an EH block which is not a landingpad.
>   cleanupret from %2563 unwind label %2277
> CleanupReturnInst must unwind to an EH block which is not a landingpad.
>   cleanupret from %2563 unwind label %2277
> The unwind destination does not have an exception handling instruction!
>   invoke void @_ZN5alloc7raw_vec17capacity_overflow17h250e7d3b3e97c20eE() #30
>           to label %2951 unwind label %3128
> The unwind destination does not have an exception handling instruction!
>   invoke void @_ZN5alloc5alloc18handle_alloc_error17h7750607dc7de1938E(i32 %2946, i32 noundef 1) #30
>           to label %2956 unwind label %3128
> The unwind destination does not have an exception handling instruction!
>   invoke void @_ZN5alloc5alloc18handle_alloc_error17h7750607dc7de1938E(i32 2, i32 noundef 1) #30
>           to label %3122 unwind label %3128
> The unwind destination does not have an exception handling instruction!
>   invoke void @_ZN5alloc5alloc18handle_alloc_error17h7750607dc7de1938E(i32 8, i32 noundef 4) #30
>           to label %3127 unwind label %3128
> CleanupPadInst not the first non-PHI instruction in the block.
>   %3133 = cleanuppad within none []
> The unwind destination does not have an exception handling instruction!
>   invoke fastcc void @"_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17hcee07cf79bbc96bcE.llvm.1999991953246885693"(ptr nonnull %132) #31 [ "funclet"(token %3157) ]
>           to label %3158 unwind label %3128
> CleanupReturnInst must unwind to an EH block which is not a landingpad.
>   cleanupret from %3157 unwind label %3128
> The unwind destination does not have an exception handling instruction!
>   invoke void @_ZN5alloc5alloc18handle_alloc_error17h7750607dc7de1938E(i32 9, i32 noundef 1) #30
>           to label %3165 unwind label %3128
> The unwind destination does not have an exception handling instruction!
>   invoke fastcc void @"_ZN4core3ptr46drop_in_place$LT$alloc..vec..Vec$LT$u8$GT$$GT$17hcee07cf79bbc96bcE.llvm.1999991953246885693"(ptr nonnull %134) #31 [ "funclet"(token %3185) ]
>           to label %3186 unwind label %3128
> CleanupReturnInst must unwind to an EH block which is not a landingpad.
>   cleanupret from %3185 unwind label %3128
> CleanupReturnInst must unwind to an EH block which is not a landingpad.
>   cleanupret from %3191 unwind label %3128
> CleanupReturnInst must unwind to an EH block which is not a landingpad.
>   cleanupret from %3309 unwind label %3128
> CleanupReturnInst must unwind to an EH block which is not a landingpad.
>   cleanupret from %3414 unwind label %3128
> CleanupReturnInst must unwind to an EH block which is not a landingpad.
>   cleanupret from %3414 unwind label %3128
> in function _ZN9rocksdict5rdict5Rdict11__getitem__17h11dfd8b0f599f69cE
> LLVM ERROR: Broken function found, compilation aborted!
> error: could not compile `RocksDict`
> 💥 maturin failed
>   Caused by: Failed to build a native library through cargo
>   Caused by: Cargo build finished with "exit code: 101": `"cargo" "rustc" "--release" "--target" "i686-pc-windows-msvc" "--message-format" "json" "--lib"`
> Error: The process 'D:\a\_temp\f0b738b8-8220-48e2-a03e-41ef287fd453\maturin.exe' failed with exit code 1
>     at ExecState._setResult (D:\a\_actions\messense\maturin-action\v1\dist\index.js:1702:25)
>     at ExecState.CheckComplete (D:\a\_actions\messense\maturin-action\v1\dist\index.js:1685:18)
>     at ChildProcess.<anonymous> (D:\a\_actions\messense\maturin-action\v1\dist\index.js:1579:27)
>     at ChildProcess.emit (node:events:527:28)
>     at maybeClose (node:internal/child_process:1092:16)
>     at Socket.<anonymous> (node:internal/child_process:451:11)
>     at Socket.emit (node:events:527:28)
>     at Pipe.<anonymous> (node:net:709:12)
> Error: The process 'D:\a\_temp\f0b738b8-8220-48e2-a03e-41ef287fd453\maturin.exe' failed with exit code 1
> 