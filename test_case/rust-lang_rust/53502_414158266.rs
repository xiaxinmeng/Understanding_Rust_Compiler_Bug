
   Compiling playground v0.0.1 (file:///playground)
warning: unreachable pattern
  --> src/main.rs:10:9
   |
9  |         DLL_PROCESS_ATTACH => PROCESS_ATTACH(),
   |         ------------------ matches any value
10 |         DLL_PROCESS_DETACH => PROCESS_DETACH(),
   |         ^^^^^^^^^^^^^^^^^^ unreachable pattern
   |
   = note: #[warn(unreachable_patterns)] on by default

warning: unreachable pattern
  --> src/main.rs:11:9
   |
9  |         DLL_PROCESS_ATTACH => PROCESS_ATTACH(),
   |         ------------------ matches any value
10 |         DLL_PROCESS_DETACH => PROCESS_DETACH(),
11 |         _=> false,
   |         ^ unreachable pattern

warning: unused variable: `DLL_PROCESS_ATTACH`
 --> src/main.rs:4:9
  |
4 |     let DLL_PROCESS_ATTACH = 1;
  |         ^^^^^^^^^^^^^^^^^^ help: consider using `_DLL_PROCESS_ATTACH` instead
  |
  = note: #[warn(unused_variables)] on by default

warning: unused variable: `DLL_PROCESS_DETACH`
 --> src/main.rs:5:9
  |
5 |     let DLL_PROCESS_DETACH = 0;
  |         ^^^^^^^^^^^^^^^^^^ help: consider using `_DLL_PROCESS_DETACH` instead

warning: unused variable: `DLL_PROCESS_ATTACH`
 --> src/main.rs:9:9
  |
9 |         DLL_PROCESS_ATTACH => PROCESS_ATTACH(),
  |         ^^^^^^^^^^^^^^^^^^ help: consider using `_DLL_PROCESS_ATTACH` instead

warning: unused variable: `DLL_PROCESS_DETACH`
  --> src/main.rs:10:9
   |
10 |         DLL_PROCESS_DETACH => PROCESS_DETACH(),
   |         ^^^^^^^^^^^^^^^^^^ help: consider using `_DLL_PROCESS_DETACH` instead

warning: variable `_hinstDLL` should have a snake case name such as `_hinst_dll`
 --> src/main.rs:2:32
  |
2 | pub extern "system" fn DllMain(_hinstDLL: u32, fdwReason: u32, _lpvReserved: *mut std::os::raw::c_void) -> bool
  |                                ^^^^^^^^^
  |
  = note: #[warn(non_snake_case)] on by default

warning: variable `fdwReason` should have a snake case name such as `fdw_reason`
 --> src/main.rs:2:48
  |
2 | pub extern "system" fn DllMain(_hinstDLL: u32, fdwReason: u32, _lpvReserved: *mut std::os::raw::c_void) -> bool
  |                                                ^^^^^^^^^

warning: variable `_lpvReserved` should have a snake case name such as `_lpv_reserved`
 --> src/main.rs:2:64
  |
2 | pub extern "system" fn DllMain(_hinstDLL: u32, fdwReason: u32, _lpvReserved: *mut std::os::raw::c_void) -> bool
  |                                                                ^^^^^^^^^^^^

warning: variable `DLL_PROCESS_ATTACH` should have a snake case name such as `dll_process_attach`
 --> src/main.rs:4:9
  |
4 |     let DLL_PROCESS_ATTACH = 1;
  |         ^^^^^^^^^^^^^^^^^^

warning: variable `DLL_PROCESS_DETACH` should have a snake case name such as `dll_process_detach`
 --> src/main.rs:5:9
  |
5 |     let DLL_PROCESS_DETACH = 0;
  |         ^^^^^^^^^^^^^^^^^^

warning: variable `DLL_PROCESS_ATTACH` should have a snake case name such as `dll_process_attach`
 --> src/main.rs:9:9
  |
9 |         DLL_PROCESS_ATTACH => PROCESS_ATTACH(),
  |         ^^^^^^^^^^^^^^^^^^

warning: variable `DLL_PROCESS_DETACH` should have a snake case name such as `dll_process_detach`
  --> src/main.rs:10:9
   |
10 |         DLL_PROCESS_DETACH => PROCESS_DETACH(),
   |         ^^^^^^^^^^^^^^^^^^

warning: function `PROCESS_ATTACH` should have a snake case name such as `process_attach`
  --> src/main.rs:15:1
   |
15 | fn PROCESS_ATTACH() -> bool { demoexec(); true }
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: function `PROCESS_DETACH` should have a snake case name such as `process_detach`
  --> src/main.rs:16:1
   |
16 | fn PROCESS_DETACH() -> bool { true }
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
