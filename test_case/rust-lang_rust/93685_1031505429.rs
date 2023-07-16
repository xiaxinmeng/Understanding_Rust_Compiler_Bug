
error[E0432]: unresolved import `winapi::um::timezoneapi`
   --> src\bootstrap\bin/rustc.rs:236:48
    |
236 |     use winapi::um::{processthreadsapi, psapi, timezoneapi};
    |                                                ^^^^^^^^^^^ no `timezoneapi` in `um`
