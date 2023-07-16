
error: unused import: `c_void`
  --> C:\bot\slave\auto-win-msvc-32-opt\build\src\librustc_trans\back\msvc\registry.rs:15:12
   |
15 | use libc::{c_void, c_long};
   |            ^^^^^^
   |
note: lint level defined here
  --> C:\bot\slave\auto-win-msvc-32-opt\build\src\librustc_trans\lib.rs:24:31
   |
24 | #![cfg_attr(not(stage0), deny(warnings))]
   |                               ^^^^^^^^
