plain
[00:50:08]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[00:50:08] [RUSTC-TIMING] alloc_system test:false 0.096
[00:50:13] [RUSTC-TIMING] alloc test:false 5.435
[00:50:13]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[00:50:13] error[E0432]: unresolved import `libc::c_int`
[00:50:13]   --> libpanic_unwind/gcc.rs:64:12
[00:50:13]    |
[00:50:13] 64 | use libc::{c_int, uintptr_t};
[00:50:13]    |            ^^^^^ no `c_int` in the root
[00:50:13] 
[00:50:13] error[E0432]: unresolved import `libc::uintptr_t`
[00:50:13]   --> libpanic_unwind/gcc.rs:64:19
[00:50:13]    |
[00:50:13] 64 | use libc::{c_int, uintptr_t};
[00:50:13]    |                   ^^^^^^^^^ no `uintptr_t` in the root
[00:50:13] 
[00:50:13] error[E0412]: cannot find type `_Unwind_Exception` in module `uw`
[00:50:13]   --> libpanic_unwind/gcc.rs:69:15
[00:50:13]    |
[00:50:13] 69 |     _uwe: uw::_Unwind_Exception,
[00:50:13]    |               ^^^^^^^^^^^^^^^^^ not found in `uw`
[00:50:13] 
[00:50:13] error[E0422]: cannot find struct, variant or union type `_Unwind_Exception` in module `uw`
[00:50:13]   --> libpanic_unwind/gcc.rs:75:19
[00:50:13]    |
[00:50:13] 75 |         _uwe: uw::_Unwind_Exception {
[00:50:13]    |                   ^^^^^^^^^^^^^^^^^ not found in `uw`
[00:50:13] 
[00:50:13] error[E0425]: cannot find value `unwinder_private_data_size` in module `uw`
[00:50:13]   --> libpanic_unwind/gcc.rs:78:30
[00:50:13]    |
[00:50:13] 78 |             private: [0; uw::unwinder_private_data_size],
[00:50:13]    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `uw`
[00:50:13] 
[00:50:13] error[E0412]: cannot find type `_Unwind_Exception` in module `uw`
[00:50:13]   --> libpanic_unwind/gcc.rs:82:64
[00:50:13]    |
[00:50:13] 82 |     let exception_param = Box::into_raw(exception) as *mut uw::_Unwind_Exception;
[00:50:13]    |                                                                ^^^^^^^^^^^^^^^^^ not found in `uw`
[00:50:13] 
[00:50:13] error[E0425]: cannot find function `_Unwind_RaiseException` in module `uw`
[00:50:13]   --> libpanic_unwind/gcc.rs:83:16
[00:50:13]    |
[00:50:13] 83 |     return uw::_Unwind_RaiseException(exception_param) as u32;
[00:50:13]    |                ^^^^^^^^^^^^^^^^^^^^^^ not found in `uw`
[00:50:13] 
[00:50:13] error[E0412]: cannot find type `_Unwind_Reason_Code` in module `uw`
[00:50:13]   --> libpanic_unwind/gcc.rs:85:55
[00:50:13]    |
[00:50:13] 85 |     extern "C" fn exception_cleanup(_unwind_code: uw::_Unwind_Reason_Code,
[00:50:13]    |                                                       ^^^^^^^^^^^^^^^^^^^ not found in `uw`
[00:50:13] 
[00:50:13] error[E0412]: cannot find type `_Unwind_Exception` in module `uw`
[00:50:13]   --> libpanic_unwind/gcc.rs:86:57
[00:50:13]    |
[00:50:13] 86 |                                     exception: *mut uw::_Unwind_Exception) {
[00:50:13]    |                                                         ^^^^^^^^^^^^^^^^^ not found in `uw`
[00:50:13] 
[00:50:13] error[E0425]: cannot find function `_Unwind_DeleteException` in module `uw`
[00:50:13]    --> libpanic_unwind/gcc.rs:100:9
[00:50:13]     |
[00:50:13] 100 |     uw::_Unwind_DeleteException(ptr as *mut _);
[00:50:13]     |         ^^^^^^^^^^^^^^^^^^^^^^^ not found in `uw`
[00:50:13] 
[00:50:13] error[E0412]: cannot find type `_Unwind_Exception_Class` in module `uw`
[00:50:13]    --> libpanic_unwind/gcc.rs:106:34
[00:50:13]     |
[00:50:13] 106 | fn rust_exception_class() -> uw::_Unwind_Exception_Class {
[00:50:13]     |                                  ^^^^^^^^^^^^^^^^^^^^^^^ not found in `uw`
[00:50:13] 
[00:50:13] error[E0412]: cannot find type `_Unwind_Action` in module `uw`
[00:50:13]    --> libpanic_unwind/gcc.rs:151:55
[00:50:13]     |
[00:50:13] 151 |                                          actions: uw::_Unwind_Action,
[00:50:13]     |                                                       ^^^^^^^^^^^^^^ not found in `uw`
[00:50:13] 
[00:50:13] error[E0412]: cannot find type `_Unwind_Exception_Class` in module `uw`
[00:50:13]    --> libpanic_unwind/gcc.rs:152:63
[00:50:13]     |
[00:50:13] 152 |                                          exception_class: uw::_Unwind_Exception_Class,
[00:50:13]     |                                                               ^^^^^^^^^^^^^^^^^^^^^^^ not found in `uw`
[00:50:13] 
[00:50:13] error[E0412]: cannot find type `_Unwind_Exception` in module `uw`
[00:50:13]    --> libpanic_unwind/gcc.rs:153:69
[00:50:13]     |
[00:50:13] 153 |                                          exception_object: *mut uw::_Unwind_Exception,
[00:50:13]     |                                                                     ^^^^^^^^^^^^^^^^^ not found in `uw`
[00:50:13] 
[00:50:13] error[E0412]: cannot find type `_Unwind_Context` in module `uw`
[00:50:13]    --> libpanic_unwind/gcc.rs:154:60
[00:50:13]     |
[00:50:13] 154 |                                          context: *mut uw::_Unwind_Context)
[00:50:13]     |                                                            ^^^^^^^^^^^^^^^ not found in `uw`
[00:50:13] 
[00:50:13] error[E0412]: cannot find type `_Unwind_Reason_Code` in module `uw`
[00:50:13]    --> libpanic_unwind/gcc.rs:155:49
[00:50:13]     |
[00:50:13] 155 |                                          -> uw::_Unwind_Reason_Code {
[00:50:13]     |                                                 ^^^^^^^^^^^^^^^^^^^ not found in `uw`
[00:50:13] 
[00:50:13] error[E0425]: cannot find value `_URC_FATAL_PHASE1_ERROR` in module `uw`
[00:50:13]    --> libpanic_unwind/gcc.rs:157:20
[00:50:13]     |
[00:50:13] 157 |         return uw::_URC_FATAL_PHASE1_ERROR;
[00:50:13]     |                    ^^^^^^^^^^^^^^^^^^^^^^^ not found in `uw`
[00:50:13] 
[00:50:13] error[E0425]: cannot find value `_URC_FATAL_PHASE1_ERROR` in module `uw`
[00:50:13]    --> libpanic_unwind/gcc.rs:161:30
[00:50:13]     |
[00:50:13] 161 |         Err(_) => return uw::_URC_FATAL_PHASE1_ERROR,
[00:50:13]     |                              ^^^^^^^^^^^^^^^^^^^^^^^ not found in `uw`
[00:50:13] 
[00:50:13] error[E0425]: cannot find value `_UA_SEARCH_PHASE` in module `uw`
[00:50:13]    --> libpanic_unwind/gcc.rs:163:29
[00:50:13]     |
[00:50:13] 163 |     if actions as i32 & uw::_UA_SEARCH_PHASE as i32 != 0 {
[00:50:13]     |                             ^^^^^^^^^^^^^^^^ not found in `uw`
[00:50:13] 
[00:50:13] error[E0425]: cannot find value `_URC_CONTINUE_UNWIND` in module `uw`
[00:50:13]    --> libpanic_unwind/gcc.rs:166:48
[00:50:13]     |
[00:50:13] 166 |             EHAction::Cleanup(_) => return uw::_URC_CONTINUE_UNWIND,
[00:50:13]     |                                                ^^^^^^^^^^^^^^^^^^^^ not found in `uw`
[00:50:13] 
[00:50:13] error[E0425]: cannot find value `_URC_HANDLER_FOUND` in module `uw`
[00:50:13]    --> libpanic_unwind/gcc.rs:167:46
[00:50:13]     |
[00:50:13] 167 |             EHAction::Catch(_) => return uw::_URC_HANDLER_FOUND,
[00:50:13]     |                                              ^^^^^^^^^^^^^^^^^^ not found in `uw`
[00:50:13] 
[00:50:13] error[E0425]: cannot find value `_URC_FATAL_PHASE1_ERROR` in module `uw`
[00:50:13]    --> libpanic_unwind/gcc.rs:168:47
[00:50:13]     |
[00:50:13] 168 |             EHAction::Terminate => return uw::_URC_FATAL_PHASE1_ERROR,
[00:50:13]     |                                               ^^^^^^^^^^^^^^^^^^^^^^^ not found in `uw`
[00:50:13] 
[00:50:13] error[E0425]: cannot find value `_URC_CONTINUE_UNWIND` in module `uw`
[00:50:13]    --> libpanic_unwind/gcc.rs:172:42
[00:50:13]     |
[00:50:13] 172 |             EHAction::None => return uw::_URC_CONTINUE_UNWIND,
[00:50:13]     |                                          ^^^^^^^^^^^^^^^^^^^^ not found in `uw`
[00:50:13] 
[00:50:13] error[E0425]: cannot find function `_Unwind_SetGR` in module `uw`
[00:50:13]    --> libpanic_unwind/gcc.rs:175:21
[00:50:13]     |
[00:50:13] 175 |                 uw::_Unwind_SetGR(context, UNWIND_DATA_REG.0, exception_object as uintptr_t);
[00:50:13]     |                     ^^^^^^^^^^^^^ not found in `uw`
[00:50:13] 
[00:50:13] error[E0425]: cannot find value `UNWIND_DATA_REG` in this scope
[00:50:13]    --> libpanic_unwind/gcc.rs:175:44
[00:50:13]     |
[00:50:13] 175 |                 uw::_Unwind_SetGR(context, UNWIND_DATA_REG.0, exception_object as uintptr_t);
[00:50:13] 
[00:50:13] 
[00:50:13] error[E0425]: cannot find function `_Unwind_SetGR` in module `uw`
[00:50:13]    --> libpanic_unwind/gcc.rs:176:21
[00:50:13]     |
[00:50:13] 176 |                 uw::_Unwind_SetGR(context, UNWIND_DATA_REG.1, 0);
[00:50:13]     |                     ^^^^^^^^^^^^^ not found in `uw`
[00:50:13] 
[00:50:13] error[E0425]: cannot find value `UNWIND_DATA_REG` in this scope
[00:50:13]    --> libpanic_unwind/gcc.rs:176:44
[00:50:13]     |
[00:50:13] 176 |                 uw::_Unwind_SetGR(context, UNWIND_DATA_REG.1, 0);
[00:50:13] 
[00:50:13] 
[00:50:13] error[E0425]: cannot find function `_Unwind_SetIP` in module `uw`
[00:50:13]    --> libpanic_unwind/gcc.rs:177:21
[00:50:13]     |
[00:50:13] 177 |                 uw::_Unwind_SetIP(context, lpad);
[00:50:13]     |                     ^^^^^^^^^^^^^ not found in `uw`
[00:50:13] 
[00:50:13] error[E0425]: cannot find value `_URC_INSTALL_CONTEXT` in module `uw`
[00:50:13]    --> libpanic_unwind/gcc.rs:178:28
[00:50:13]     |
[00:50:13] 178 |                 return uw::_URC_INSTALL_CONTEXT;
[00:50:13]     |                            ^^^^^^^^^^^^^^^^^^^^ not found in `uw`
[00:50:13] 
[00:50:13] error[E0425]: cannot find value `_URC_FATAL_PHASE2_ERROR` in module `uw`
[00:50:13]    --> libpanic_unwind/gcc.rs:180:47
[00:50:13]     |
[00:50:13] 180 |             EHAction::Terminate => return uw::_URC_FATAL_PHASE2_ERROR,
[00:50:13]     |                                               ^^^^^^^^^^^^^^^^^^^^^^^ not found in `uw`
[00:50:13] 
[00:50:13] error[E0412]: cannot find type `_Unwind_Context` in module `uw`
[00:50:13]    --> libpanic_unwind/gcc.rs:269:44
[00:50:13]     |
[00:50:13] 269 | unsafe fn find_eh_action(context: *mut uw::_Unwind_Context)
[00:50:13]     |                                            ^^^^^^^^^^^^^^^ not found in `uw`
[00:50:13] 
[00:50:13] error[E0425]: cannot find function `_Unwind_GetLanguageSpecificData` in module `uw`
[00:50:13]    --> libpanic_unwind/gcc.rs:272:20
[00:50:13]     |
[00:50:13] 272 |     let lsda = uw::_Unwind_GetLanguageSpecificData(context) as *const u8;
[00:50:13]     |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `uw`
[00:50:13] 
[00:50:13] error[E0425]: cannot find function `_Unwind_GetIPInfo` in module `uw`
[00:50:13]    --> libpanic_unwind/gcc.rs:274:18
[00:50:13]     |
[00:50:13] 274 |     let ip = uw::_Unwind_GetIPInfo(context, &mut ip_before_instr);
[00:50:13]     |                  ^^^^^^^^^^^^^^^^^ not found in `uw`
[00:50:13] 
[00:50:13] error[E0425]: cannot find function `_Unwind_GetRegionStart` in module `uw`
[00:50:13]    --> libpanic_unwind/gcc.rs:279:25
[00:50:13]     |
[00:50:13] 279 |         func_start: uw::_Unwind_GetRegionStart(context),
[00:50:13]     |                         ^^^^^^^^^^^^^^^^^^^^^^ not found in `uw`
[00:50:13] 
[00:50:13] error[E0425]: cannot find function `_Unwind_GetTextRelBase` in module `uw`
[00:50:13]    --> libpanic_unwind/gcc.rs:280:33
[00:50:13]     |
[00:50:13] 280 |         get_text_start: &|| uw::_Unwind_GetTextRelBase(context),
[00:50:13]     |                                 ^^^^^^^^^^^^^^^^^^^^^^ not found in `uw`
[00:50:13] 
[00:50:13] error[E0425]: cannot find function `_Unwind_GetDataRelBase` in module `uw`
[00:50:13]    --> libpanic_unwind/gcc.rs:281:33
[00:50:13]     |
[00:50:13] 281 |         get_data_start: &|| uw::_Unwind_GetDataRelBase(context),
[00:50:13]     |                                 ^^^^^^^^^^^^^^^^^^^^^^ not found in `uw`
[00:50:13] error: aborting due to 36 previous errors
[00:50:13] 
[00:50:13] Some errors occurred: E0412, E0422, E0425, E0432.
[00:50:13] For more information about an error, try `rustc --explain E0412`.
[00:50:13] For more information about an error, try `rustc --explain E0412`.
[00:50:13] [RUSTC-TIMING] panic_unwind test:false 0.184
[00:50:13] error: Could not compile `panic_unwind`.
[00:50:13] Caused by:
[00:50:13] Caused by:
[00:50:13]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name panic_unwind libpanic_unwind/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=fd125611def02e2b -C extra-filename=-fd125611def02e2b --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/wasm32-unknown-unknown/release/deps --target wasm32-unknown-unknown -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/wasm32-unknown-unknown/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/deps --extern alloc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/wasm32-unknown-unknown/release/deps/liballoc-a90a20d75fb2d756.rlib --extern compiler_builtins=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/wasm32-unknown-unknown/release/deps/libcompiler_builtins-ce73107aecc35a63.rlib --extern core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/wasm32-unknown-unknown/release/deps/libcore-b33d847693b19528.rlib --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/wasm32-unknown-unknown/release/deps/liblibc-986eedb245ac8126.rlib --extern unwind=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/wasm32-unknown-unknown/release/deps/libunwind-136f2251422396fe.rlib` (exit code: 1)
[00:50:13] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "wasm32-unknown-unknown" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:50:13] expected success, got: exit code: 101
[00:50:13] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:50:13] travis_fold:end:stage2-std

[00:50:13] travis_time:end:stage2-std:start=1534373628884321155,finish=1534373662749322906,duration=33865001751

---
The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
travis_fold:start:after_failure.1
travis_time:start:03a83600
$ sudo tail -n 500 /var/log/syslog
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] ACPI: FACS 0x00000000BFFFCF00 000040
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] ACPI: SSDT 0x00000000BFFF65F0 00690D (v01 Google GOOGSSDT 00000001 GOOG 00000001)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] ACPI: APIC 0x00000000BFFF5D10 000086 (v01 Google GOOGAPIC 00000001 GOOG 00000001)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] ACPI: WAET 0x00000000BFFF5CE0 000028 (v01 Google GOOGWAET 00000001 GOOG 00000001)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] ACPI: SRAT 0x00000000BFFF4C30 0000E8 (v01 Google GOOGSRAT 00000001 GOOG 00000001)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x00 -> Node 0
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x01 -> Node 0
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x02 -> Node 0
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] SRAT: PXM 0 -> APIC 0x03 -> Node 0
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00000000-0x0009ffff]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x00100000-0xbfffffff]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] SRAT: Node 0 PXM 0 [mem 0x100000000-0x3ffffffff]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0x0009ffff] + [mem 0x00100000-0xbfffffff] -> [mem 0x00000000-0xbfffffff]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] NUMA: Node 0 [mem 0x00000000-0xbfffffff] + [mem 0x100000000-0x3ffffffff] -> [mem 0x00000000-0x3ffffffff]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] NODE_DATA(0) allocated [mem 0x3ffff9000-0x3ffffdfff]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] kvm-clock: Using msrs 4b564d01 and 4b564d00
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] kvm-clock: cpu 0, msr 3:ffff1001, primary cpu clock
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] kvm-clock: using sched offset of 2543923505 cycles
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] clocksource: kvm-clock: mask: 0xffffffffffffffff max_cycles: 0x1cd42e4dffb, max_idle_ns: 881590591483 ns
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] Zone ranges:
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000]   Device   empty
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] Movable zone start for each node
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] Early memory node ranges
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xfffbc000-0xffffffff]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] e820: [mem 0xc0000000-0xfffbbfff] available for PCI devices
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] Booting paravirtualized kernel on KVM
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645519600211568 ns
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] setup_percpu: NR_CPUS:512 nr_cpumask_bits:512 nr_cpu_ids:4 nr_node_ids:1
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] PERCPU: Embedded 34 pages/cpu @ffff8803ffc00000 s98392 r8192 d32680 u524288
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] pcpu-alloc: s98392 r8192 d32680 u524288 alloc=1*2097152
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] pcpu-alloc: [0] 0 1 2 3 
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] Built 1 zonelists in Node order, mobility grouping on.  Total pages: 3870588
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] Policy zone: Normal
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] Kernel command line: BOOT_IMAGE=/boot/vmlinuz-4.4.0-101-generic root=UUID=752b4ef1-0512-4cae-b541-f03ffd29be1b ro cgroup_enable=memory swapaccount=1 apparmor=0 console=ttyS0 console=ttyS0
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] console [ttyS0] enabled
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.000000] tsc: Detected 2500.000 MHz processor
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.517651] Calibrating delay loop (skipped) preset value.. 5000.00 BogoMIPS (lpj=10000000)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.520522] pid_max: default: 32768 minimum: 301
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.522109] ACPI: Core revision 20150930
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.530242] ACPI: 2 ACPI AML tables successfully acquired and loaded
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.533188] Security Framework initialized
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.534758] Yama: becoming mindful.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.535877] AppArmor: AppArmor disabled by boot time parameter
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.539528] Dentry cache hash table entries: 2097152 (order: 12, 16777216 bytes)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.551322] Inode-cache hash table entries: 1048576 (order: 11, 8388608 bytes)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.557812] Mount-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.560540] Mountpoint-cache hash table entries: 32768 (order: 6, 262144 bytes)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.563825] Initializing cgroup subsys io
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.565458] Initializing cgroup subsys memory
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.566851] Initializing cgroup subsys devices
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.568205] Initializing cgroup subsys freezer
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.569836] Initializing cgroup subsys net_cls
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.571269] Initializing cgroup subsys perf_event
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.573507] Initializing cgroup subsys net_prio
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.575201] Initializing cgroup subsys hugetlb
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.577060] Initializing cgroup subsys pids
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.578389] CPU: Physical Processor ID: 0
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.580465] CPU: Processor Core ID: 0
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.582865] mce: CPU supports 32 MCE banks
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.584619] Last level iTLB entries: 4KB 512, 2MB 8, 4MB 8
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.586735] Last level dTLB entries: 4KB 512, 2MB 0, 4MB 0, 1GB 4
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.591382] Freeing SMP alternatives memory: 32K
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.604977] ftrace: allocating 32185 entries in 126 pages
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.668172] smpboot: APIC(0) Converting physical 0 to logical package 0
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.670296] smpboot: Max logical packages: 2
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.672577] x2apic enabled
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.675484] Switched APIC routing to physical x2apic.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.681306] ..TIMER: vector=0x30 apic1=0 pin1=0 apic2=-1 pin2=-1
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.787981] smpboot: CPU0: Intel(R) Xeon(R) CPU @ 2.50GHz (family: 0x6, model: 0x3e, stepping: 0x4)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.791587] Performance Events: unsupported p6 CPU model 62 no PMU driver, software events only.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.796690] x86: Booting SMP configuration:
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.798519] .... node  #0, CPUs:      #1
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.800048] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.806728]  #2
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.807598] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.814494]  #3
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.815314] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.821498] x86: Booted up 1 node, 4 CPUs
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.823039] smpboot: Total of 4 processors activated (20000.00 BogoMIPS)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.827358] devtmpfs: initialized
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.833019] evm: security.selinux
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.834161] evm: security.SMACK64
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.835450] evm: security.SMACK64EXEC
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.836554] evm: security.SMACK64TRANSMUTE
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.837739] evm: security.SMACK64MMAP
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.839786] evm: security.ima
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.841156] evm: security.capability
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.843662] clocksource: jiffies: mask: 0xffffffff max_cycles: 0xffffffff, max_idle_ns: 7645041785100000 ns
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.848616] futex hash table entries: 1024 (order: 4, 65536 bytes)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.851811] pinctrl core: initialized pinctrl subsystem
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.853923] RTC time: 22:02:54, date: 08/15/18
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.856788] NET: Registered protocol family 16
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.868035] cpuidle: using governor ladder
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.880119] cpuidle: using governor menu
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.882012] PCCT header not found.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.883561] ACPI: bus type PCI registered
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.885396] acpiphp: ACPI Hot Plug PCI Controller Driver version: 0.5
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.889001] PCI: Using configuration type 1 for base access
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.905960] ACPI: Added _OSI(Module Device)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.908560] ACPI: Added _OSI(Processor Device)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.910984] ACPI: Added _OSI(3.0 _SCP Extensions)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.912801] ACPI: Added _OSI(Processor Aggregator Device)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.918249] ACPI: Executed 2 blocks of module-level executable AML code
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.944056] ACPI: Interpreter enabled
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.947049] ACPI: (supports S0 S3 S4 S5)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.949193] ACPI: Using IOAPIC for interrupt routing
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.951178] PCI: Using host bridge windows from ACPI; if necessary, use "pci=nocrs" and report a bug
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.984167] ACPI: PCI Root Bridge [PCI0] (domain 0000 [bus 00-ff])
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.988073] acpi PNP0A03:00: _OSC: OS supports [ASPM ClockPM Segments MSI]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.991714] acpi PNP0A03:00: _OSC failed (AE_NOT_FOUND); disabling ASPM
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    0.994982] acpi PNP0A03:00: fail to add MMCONFIG information, can't access extended PCI configuration space under this bridge.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.000597] PCI host bridge to bus 0000:00
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.003039] pci_bus 0000:00: root bus resource [io  0x0000-0x0cf7 window]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.006176] pci_bus 0000:00: root bus resource [io  0x0d00-0xffff window]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.009273] pci_bus 0000:00: root bus resource [mem 0x000a0000-0x000bffff window]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.012914] pci_bus 0000:00: root bus resource [mem 0xc0000000-0xfebfffff window]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.016421] pci_bus 0000:00: root bus resource [bus 00-ff]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.019048] pci 0000:00:00.0: [8086:1237] type 00 class 0x060000
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.019590] pci 0000:00:01.0: [8086:7110] type 00 class 0x060100
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.046054] pci 0000:00:01.3: [8086:7113] type 00 class 0x068000
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.073409] pci 0000:00:01.3: quirk: [io  0xb000-0xb03f] claimed by PIIX4 ACPI
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.076483] pci 0000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.087469] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.097271] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.121508] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.131379] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.139908] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.167287] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.172343] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.178401] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.183618] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.190253] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.214032] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.217103] vgaarb: loaded
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.218575] SCSI subsystem initialized
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.220849] libata version 3.00 loaded.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.220882] ACPI: bus type USB registered
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.222859] usbcore: registered new interface driver usbfs
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.225651] usbcore: registered new interface driver hub
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.228565] usbcore: registered new device driver usb
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.230672] ioremap error for 0xbfffd000-0xc0000000, requested 0x2, got 0x0
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.233357] dmi: Firmware registration failed.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.235426] PCI: Using ACPI for IRQ routing
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.237134] PCI: pci_cache_line_size set to 64 bytes
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.237241] e820: reserve RAM buffer [mem 0x0009fc00-0x0009ffff]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.237243] e820: reserve RAM buffer [mem 0xbfff3000-0xbfffffff]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.237388] NetLabel: Initializing
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.238706] NetLabel:  domain hash size = 128
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.240613] NetLabel:  protocols = UNLABELED CIPSOv4
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.242653] NetLabel:  unlabeled traffic allowed by default
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.245242] amd_nb: Cannot enumerate AMD northbridges
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.247596] clocksource: Switched to clocksource kvm-clock
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.258132] pnp: PnP ACPI init
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.259643] pnp 00:00: Plug and Play ACPI device, IDs PNP0b00 (active)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.259709] pnp 00:01: Plug and Play ACPI device, IDs PNP0303 (active)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.259752] pnp 00:02: Plug and Play ACPI device, IDs PNP0f13 (active)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.259802] pnp 00:03: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.259872] pnp 00:04: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.259915] pnp 00:05: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.259987] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.260166] pnp: PnP ACPI: found 7 devices
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.270180] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.274781] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.274784] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.274785] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.274787] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.274854] NET: Registered protocol family 2
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.277564] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.281148] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.285388] TCP: Hash tables configured (established 131072 bind 65536)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.287687] UDP hash table entries: 8192 (order: 6, 262144 bytes)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.289518] UDP-Lite hash table entries: 8192 (order: 6, 262144 bytes)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.292740] NET: Registered protocol family 1
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.294615] pci 0000:00:00.0: Limiting direct PCI/PCI transfers
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.296582] PCI: CLS 0 bytes, default 64
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    1.296655] Unpacking initramfs...
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.497935] Freeing initrd memory: 21432K
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.499808] PCI-DMA: Using software bounce buffering for IO (SWIOTLB)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.502242] software IO TLB [mem 0xbbff3000-0xbfff3000] (64MB) mapped at [ffff8800bbff3000-ffff8800bfff2fff]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.505639] RAPL PMU detected, API unit is 2^-32 Joules, 3 fixed counters 10737418240 ms ovfl timer
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.508903] hw unit of domain pp0-core 2^-0 Joules
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.511532] hw unit of domain package 2^-0 Joules
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.513325] hw unit of domain dram 2^-0 Joules
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.515578] Scanning for low memory corruption every 60 seconds
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.519072] audit: initializing netlink subsys (disabled)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.521972] audit: type=2000 audit(1534370576.266:1): initialized
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.525026] Initialise system trusted keyring
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.527008] HugeTLB registered 1 GB page size, pre-allocated 0 pages
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.529453] HugeTLB registered 2 MB page size, pre-allocated 0 pages
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.533389] zbud: loaded
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.534724] VFS: Disk quotas dquot_6.6.0
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.536377] VFS: Dquot-cache hash table entries: 512 (order 0, 4096 bytes)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.539176] squashfs: version 4.0 (2009/01/31) Phillip Lougher
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.542510] fuse init (API version 7.23)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.544650] Key type big_key registered
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.546295] Allocating IMA MOK and blacklist keyrings.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.553556] Key type asymmetric registered
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.555024] Asymmetric key parser 'x509' registered
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.557144] Block layer SCSI generic (bsg) driver version 0.4 loaded (major 249)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.560320] io scheduler noop registered
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.561666] io scheduler deadline registered (default)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.563508] io scheduler cfq registered
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.565317] pci_hotplug: PCI Hot Plug PCI Core version: 0.5
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.567241] pciehp: PCI Express Hot Plug Controller Driver version: 0.4
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.569900] intel_idle: does not run on family 6 model 62
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.570025] input: Power Button as /devices/LNXSYSTM:00/LNXPWRBN:00/input/input0
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.572403] ACPI: Power Button [PWRF]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.573700] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.576506] ACPI: Sleep Button [SLPF]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.578254] GHES: HEST is not enabled!
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.582188] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.584371] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.595360] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.597882] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.608355] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.632026] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.657031] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.682181] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.707012] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) is a 16550A
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.712146] Linux agpgart interface v0.103
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.718524] loop: module loaded
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.719970] libphy: Fixed MDIO Bus: probed
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.721563] tun: Universal TUN/TAP device driver, 1.6
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.723719] tun: (C) 1999-2004 Max Krasnyansky <maxk@qualcomm.com>
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.781097] PPP generic driver version 2.4.2
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.783088] ehci_hcd: USB 2.0 'Enhanced' Host Controller (EHCI) Driver
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.785426] ehci-pci: EHCI PCI platform driver
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.787534] ehci-platform: EHCI generic platform driver
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.789518] ohci_hcd: USB 1.1 'Open' Host Controller (OHCI) Driver
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.792057] ohci-pci: OHCI PCI platform driver
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.793946] ohci-platform: OHCI generic platform driver
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.795982] uhci_hcd: USB Universal Host Controller Interface driver
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.798433] i8042: PNP: PS/2 Controller [PNP0303:KBD,PNP0f13:MOU] at 0x60,0x64 irq 1,12
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.803354] i8042: Warning: Keylock active
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.806414] serio: i8042 KBD port at 0x60,0x64 irq 1
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.808699] serio: i8042 AUX port at 0x60,0x64 irq 12
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.811036] mousedev: PS/2 mouse device common for all mice
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.814124] rtc_cmos 00:00: RTC can wake from S4
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.816155] rtc_cmos 00:00: rtc core: registered rtc_cmos as rtc0
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.819148] rtc_cmos 00:00: alarms up to one day, 114 bytes nvram
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.821626] i2c /dev entries driver
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.823533] device-mapper: uevent: version 1.0.3
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.825684] device-mapper: ioctl: 4.34.0-ioctl (2015-10-28) initialised: dm-devel@redhat.com
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.829340] ledtrig-cpu: registered to indicate activity on CPUs
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.833505] NET: Registered protocol family 10
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.836519] NET: Registered protocol family 17
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.838779] Key type dns_resolver registered
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.841707] microcode: CPU0 sig=0x306e4, pf=0x1, revision=0x1
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.844280] microcode: CPU1 sig=0x306e4, pf=0x1, revision=0x1
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.847497] microcode: CPU2 sig=0x306e4, pf=0x1, revision=0x1
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.849884] microcode: CPU3 sig=0x306e4, pf=0x1, revision=0x1
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.852978] microcode: Microcode Update Driver: v2.01 <tigran@aivazian.fsnet.co.uk>, Peter Oruba
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.857608] registered taskstats version 1
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.859393] Loading compiled-in X.509 certificates
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.862279] Loaded X.509 cert 'Build time autogenerated kernel key: 56232512f0584176d25dbc659499b922e518c1c1'
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.866762] zswap: loaded using pool lzo/zbud
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.871211] Key type trusted registered
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.878631] Key type encrypted registered
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.880379] ima: No TPM chip found, activating TPM-bypass!
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.882367] evm: HMAC attrs: 0x1
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.885090]   Magic number: 14:435:47
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.887378] rtc_cmos 00:00: setting system clock to 2018-08-15 22:02:57 UTC (1534370577)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.891360] BIOS EDD facility v0.16 2004-Jun-25, 0 devices found
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.894084] EDD information not available.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.895834] PM: Hibernation image not present or could not be loaded.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.897748] Freeing unused kernel memory: 1496K
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.899130] Write protecting the kernel read-only data: 14336k
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.902357] Freeing unused kernel memory: 1956K
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.904532] Freeing unused kernel memory: 92K
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    3.925136] systemd-udevd[118]: starting version 204
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    4.002339] scsi host0: Virtio SCSI HBA
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    4.008761] scsi 0:0:1:0: Direct-Access     Google   PersistentDisk   1    PQ: 0 ANSI: 6
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    4.012025] AVX version of gcm_enc/dec engaged.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    4.013420] AES CTR mode by8 optimization enabled
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    4.015983] input: AT Translated Set 2 keyboard as /devices/platform/i8042/serio0/input/input2
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    4.076136] sd 0:0:1:0: Attached scsi generic sg0 type 0
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    4.076210] sd 0:0:1:0: [sda] 62914560 512-byte logical blocks: (32.2 GB/30.0 GiB)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    4.076212] sd 0:0:1:0: [sda] 4096-byte physical blocks
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    4.076404] sd 0:0:1:0: [sda] Write Protect is off
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    4.076406] sd 0:0:1:0: [sda] Mode Sense: 1f 00 00 08
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    4.076455] sd 0:0:1:0: [sda] Write cache: enabled, read cache: enabled, doesn't support DPO or FUA
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    4.079142]  sda: sda1
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    4.080758] sd 0:0:1:0: [sda] Attached SCSI disk
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    4.511793] tsc: Refined TSC clocksource calibration: 2499.766 MHz
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    4.514180] clocksource: tsc: mask: 0xffffffffffffffff max_cycles: 0x24085cabc78, max_idle_ns: 440795288552 ns
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    4.852836] input: ImExPS/2 Generic Explorer Mouse as /devices/platform/i8042/serio1/input/input4
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    6.983969] floppy0: no floppy controllers found
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    8.175642] raid6: sse2x1   gen()  8606 MB/s
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    8.243759] raid6: sse2x1   xor()  6503 MB/s
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    8.311638] raid6: sse2x2   gen() 10854 MB/s
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    8.379658] raid6: sse2x2   xor()  7620 MB/s
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    8.447698] raid6: sse2x4   gen() 12459 MB/s
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    8.515659] raid6: sse2x4   xor()  8463 MB/s
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    8.519119] raid6: using algorithm sse2x4 gen() 12459 MB/s
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    8.521273] raid6: .... xor() 8463 MB/s, rmw enabled
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    8.523430] raid6: using ssse3x2 recovery algorithm
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    8.527203] xor: automatically using best checksumming function:
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    8.567605]    avx       : 21480.000 MB/sec
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    8.585496] Btrfs loaded
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    8.655309] EXT4-fs (sda1): INFO: recovery required on readonly filesystem
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    8.658224] EXT4-fs (sda1): write access will be enabled during recovery
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    8.737190] EXT4-fs (sda1): orphan cleanup on readonly fs
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    8.754719] EXT4-fs (sda1): 6 orphan inodes deleted
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    8.757666] EXT4-fs (sda1): recovery complete
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    8.766586] EXT4-fs (sda1): mounted filesystem with ordered data mode. Opts: (null)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    9.019783] random: init: uninitialized urandom read (12 bytes read, 22 bits of entropy available)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    9.158462] random: mountall: uninitialized urandom read (12 bytes read, 26 bits of entropy available)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    9.217630] EXT4-fs (sda1): re-mounted. Opts: (null)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [    9.429957] random: cloud-init: uninitialized urandom read (32 bytes read, 32 bits of entropy available)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [   10.057541] random: cloud-init: uninitialized urandom read (32 bytes read, 40 bits of entropy available)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [   10.226064] systemd-udevd[702]: starting version 204
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [   10.359696] piix4_smbus 0000:00:01.3: SMBus base address uninitialized - upgrade BIOS or use force_addr=0xaddr
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [   10.470063] ppdev: user-space parallel port driver
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [   10.576589] random: mktemp: uninitialized urandom read (6 bytes read, 50 bits of entropy available)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [   10.640127] random: mktemp: uninitialized urandom read (6 bytes read, 51 bits of entropy available)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [   10.713854] random: cloud-init: uninitialized urandom read (32 bytes read, 51 bits of entropy available)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [   10.879759] random: cloud-init: uninitialized urandom read (32 bytes read, 51 bits of entropy available)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [   11.117276] random: mktemp: uninitialized urandom read (12 bytes read, 54 bits of entropy available)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [   11.190337] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [   11.269100] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [   11.304000] EXT4-fs (sda1): resized filesystem to 7864064
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [   11.559536] init: failsafe main process (1093) killed by TERM signal
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 instance-setup: INFO Running set_multiqueue.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 instance-setup: INFO Set channels for eth0 to 4.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 instance-setup: INFO /proc/irq/29/smp_affinity_list: real affinity 2
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 instance-setup: INFO Setting /proc/irq/30/smp_affinity_list to 2 for device virtio1.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 instance-setup: INFO /proc/irq/30/smp_affinity_list: real affinity 2
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 instance-setup: INFO Setting /proc/irq/31/smp_affinity_list to 3 for device virtio1.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 instance-setup: INFO /proc/irq/31/smp_affinity_list: real affinity 3
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 instance-setup: INFO Setting /proc/irq/32/smp_affinity_list to 3 for device virtio1.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 instance-setup: INFO /proc/irq/32/smp_affinity_list: real affinity 3
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 instance-setup: INFO Queue 0 XPS=1 for /sys/class/net/eth0/queues/tx-0/xps_cpus
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 instance-setup: INFO Queue 1 XPS=2 for /sys/class/net/eth0/queues/tx-1/xps_cpus
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 instance-setup: INFO Queue 2 XPS=4 for /sys/class/net/eth0/queues/tx-2/xps_cpus
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 instance-setup: INFO Queue 3 XPS=8 for /sys/class/net/eth0/queues/tx-3/xps_cpus
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 google-ip-forwarding: INFO Starting Google IP Forwarding daemon.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 google-accounts: INFO Starting Google Accounts daemon.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 google-accounts: INFO Creating a new user account for me.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 google-accounts: INFO Created user account me.
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [   12.367594] random: nonblocking pool is initialized
Aug 15 22:03:05 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 google-accounts: INFO Creating a new user account for henrik.
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 google-accounts: INFO Created user account henrik.
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 google-accounts: INFO Creating a new user account for emma.
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 google-accounts: INFO Created user account emma.
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 google-accounts: INFO Creating a new user account for igor.
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 google-accounts: INFO Created user account igor.
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 google-accounts: INFO Creating a new user account for konstantinhaase.
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 google-accounts: INFO Created user account konstantinhaase.
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 google-accounts: INFO Creating a new user account for aj.
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 google-accounts: INFO Created user account aj.
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 google-accounts: INFO Creating a new user account for solarce.
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 cron[1443]: (CRON) INFO (pidfile fd = 3)
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 cron[1491]: (CRON) STARTUP (fork ok)
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 cron[1491]: (CRON) INFO (Running @reboot jobs)
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 google-accounts: INFO Created user account solarce.
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 google-accounts: INFO Creating a new user account for asari.
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 acpid: starting up with netlink and the input layer
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 acpid: 1 rule loaded
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 acpid: waiting for events: event logging is off
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 google-accounts: INFO Created user account asari.
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 haveged: haveged starting up
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 google-accounts: INFO Creating a new user account for bogdana.
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 pollinate: To re-seed this system again, use the -r|--reseed option
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 google-clock-skew: INFO Synced system time with hardware clock.
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 google-accounts: INFO Created user account bogdana.
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [   12.878856] ip_tables: (C) 2000-2006 Netfilter Core Team
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [   12.893269] ip6_tables: (C) 2000-2006 Netfilter Core Team
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 google-accounts: INFO Creating a new user account for konstantin.
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 google-accounts: INFO Created user account konstantin.
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 google-accounts: INFO Creating a new user account for carmen.
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [   12.961111] bridge: automatic filtering via arp/ip/ip6tables has been deprecated. Update your scripts to load br_netfilter if you need this.
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [   12.966497] Bridge firewalling registered
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [   12.979139] nf_conntrack version 0.5.0 (65536 buckets, 262144 max)
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 google-accounts: INFO Created user account carmen.
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 google-accounts: INFO Creating a new user account for maria.
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [   13.066050] Initializing XFRM netlink socket
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 google-accounts: INFO Created user account maria.
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [   13.078094] Netfilter messages via NETLINK v0.30.
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [   13.081336] ctnetlink v0.93: registering with nfnetlink.
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 google-accounts: INFO Removing user packer.
Aug 15 22:03:06 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [   13.407831] floppy0: no floppy controllers found
Aug 15 22:03:29 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 ntpdate[1864]: adjust time server 169.254.169.254 offset 0.030419 sec
Aug 15 22:03:36 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 ntpd[1899]: ntpd 4.2.6p5@1.2349-o Wed Jul 12 12:22:58 UTC 2017 (1)
Aug 15 22:03:36 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 ntpd[1900]: proto: precision = 0.103 usec
Aug 15 22:03:36 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 ntpd[1900]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 15 22:03:36 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 ntpd[1900]: ntp_io: estimated max descriptors: 1024, initial socket boundary: 16
Aug 15 22:03:36 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 ntpd[1900]: Listen and drop on 0 v4wildcard 0.0.0.0 UDP 123
Aug 15 22:03:36 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 ntpd[1900]: Listen and drop on 1 v6wildcard :: UDP 123
Aug 15 22:03:36 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 ntpd[1900]: Listen normally on 2 lo 127.0.0.1 UDP 123
Aug 15 22:03:36 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 ntpd[1900]: Listen normally on 3 eth0 10.20.2.99 UDP 123
Aug 15 22:03:36 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 ntpd[1900]: Listen normally on 4 docker0 172.17.0.1 UDP 123
Aug 15 22:03:36 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 ntpd[1900]: peers refreshed
Aug 15 22:03:36 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 ntpd[1900]: Listening on routing socket on fd #21 for interface updates
Aug 15 22:03:36 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [   43.120180] init: plymouth-upstart-bridge main process ended, respawning
Aug 15 22:03:36 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 startup-script: INFO Found startup-script in metadata.
Aug 15 22:03:36 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 startup-script: INFO startup-script: warning: commands will be executed using /bin/sh
Aug 15 22:03:36 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 startup-script: INFO startup-script: job 1 at Thu Aug 16 01:13:00 2018
Aug 15 22:03:36 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 startup-script: INFO startup-script: Return code 0.
Aug 15 22:03:36 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 startup-script: INFO startup-script: Return code 0.
Aug 15 22:03:36 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 startup-script: INFO Finished running startup scripts.
Aug 15 22:03:36 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 ec2: 
Aug 15 22:03:36 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 ec2: #############################################################
Aug 15 22:03:36 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 ec2: -----BEGIN SSH HOST KEY FINGERPRINTS-----
Aug 15 22:03:36 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 ec2: 1024 2a:c8:23:31:d5:ab:1c:6c:69:2e:15:2c:be:0d:9d:65  root@travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 (DSA)
Aug 15 22:03:36 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 ec2: 256 86:0a:26:40:12:95:87:dc:35:3a:c1:d8:09:d3:00:00  root@travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 (ECDSA)
Aug 15 22:03:36 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 ec2: 256 e0:35:7b:6d:87:8e:06:5b:93:72:bb:a7:cd:65:0d:e1  root@travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 (ED25519)
Aug 15 22:03:36 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 ec2: 2048 74:ac:be:9e:46:df:9b:ad:59:29:d5:3b:da:48:66:aa  root@travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 (RSA)
Aug 15 22:03:36 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 15 22:03:36 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 ec2: #############################################################
Aug 15 22:04:08 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [   75.176088] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 15 22:05:59 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [  186.155417] device veth7bbc196 entered promiscuous mode
Aug 15 22:05:59 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [  186.155563] docker0: port 1(veth7bbc196) entered forwarding state
Aug 15 22:05:59 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [  186.155572] docker0: port 1(veth7bbc196) entered forwarding state
Aug 15 22:05:59 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [  186.156144] docker0: port 1(veth7bbc196) entered disabled state
Aug 15 22:05:59 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [  186.260911] cgroup: docker-runc (4977) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 15 22:05:59 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [  186.260915] cgroup: "memory" requires setting use_hierarchy to 1 on the root
Aug 15 22:05:59 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [  186.344167] eth0: renamed from veth9223999
Aug 15 22:05:59 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [  186.384735] IPv6: ADDRCONF(NETDEV_UP): eth0: link is not ready
Aug 15 22:05:59 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [  186.386342] docker0: port 1(veth7bbc196) entered forwarding state
Aug 15 22:05:59 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [  186.386356] docker0: port 1(veth7bbc196) entered forwarding state
Aug 15 22:05:59 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [  186.386406] IPv6: ADDRCONF(NETDEV_CHANGE): docker0: link becomes ready
Aug 15 22:06:03 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 ntpd[1900]: Listen normally on 5 docker0 fe80::42:33ff:feed:8ad0 UDP 123
Aug 15 22:06:03 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 ntpd[1900]: Listen normally on 6 docker0 fe80::1 UDP 123
Aug 15 22:06:03 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 ntpd[1900]: Listen normally on 7 docker0 fd9a:8454:6789:13f7::1 UDP 123
Aug 15 22:06:03 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 ntpd[1900]: peers refreshed
Aug 15 22:06:03 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 ntpd[1900]: new interface(s) found: waking up resolver
Aug 15 22:06:14 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 kernel: [  201.430574] docker0: port 1(veth7bbc196) entered forwarding state
Aug 15 22:17:01 travis-job-14e97dc4-a0d7-4221-b678-1699923dacb7 CRON[12388]: (root) CMD (   cd / && run-parts --report /etc/cron.hourly)
---
travis_time:end:028f216f:start=1534373663899200593,finish=1534373663906868451,duration=7667858
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:12326ca4
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:10e39846
travis_time:start:10e39846
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.6
travis_fold:start:after_failure.7
travis_time:start:0d9c6c5a
$ dmesg | grep -i kill
