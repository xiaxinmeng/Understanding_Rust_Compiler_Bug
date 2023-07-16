plain
[00:47:05] ....................................................................................................
[00:47:09] ....................................................................................................
[00:47:11] .......................i............................................................................
[00:47:14] ....................................................................................................
[00:47:16] ........................................................................iiiiiiiii...................
[00:47:22] ....................................................................................................
[00:47:25] ....................................................................................................
[00:47:28] .....................................................i..............................................
[00:47:31] ....................................................................................................
---
[01:07:28] .................................................................................................... 600/940
[01:07:39] ......................................................................iiii.......................... 700/940
[01:07:53] .................................................................................................... 800/940
[01:07:58] .......................................................................................iiii......... 900/940
[01:08:02] ..............................F..F......
[01:08:02] 
[01:08:02] 
[01:08:02] ---- time/mod.rs - time::SystemTime::display_iso_8601 (line 386) stdout ----
[01:08:02] error[E0658]: use of unstable library feature 'system_time_display_iso_8601' (see issue #53891)
[01:08:02]  --> time/mod.rs:390:42
[01:08:02]   |
[01:08:02] 7 |     format!("{}", SystemTime::UNIX_EPOCH.display_iso_8601()));
[01:08:02]   |
[01:08:02]   |
[01:08:02]   = help: add #![feature(system_time_display_iso_8601)] to the crate attributes to enable
[01:08:02] 
[01:08:02] error[E0658]: use of unstable library feature 'system_time_display_iso_8601' (see issue #53891)
[01:08:02]  --> time/mod.rs:392:45
[01:08:02]   |
[01:08:02] 9 |     format!("{:.3}", SystemTime::UNIX_EPOCH.display_iso_8601()));
[01:08:02]   |
[01:08:02]   |
[01:08:02]   = help: add #![feature(system_time_display_iso_8601)] to the crate attributes to enable
[01:08:02] 
[01:08:02] error[E0658]: use of unstable library feature 'system_time_display_iso_8601' (see issue #53891)
[01:08:02]   --> time/mod.rs:394:45
[01:08:02]    |
[01:08:02] 11 |     format!("{:.0}", SystemTime::UNIX_EPOCH.display_iso_8601()));
[01:08:02]    |
[01:08:02]    |
[01:08:02]    = help: add #![feature(system_time_display_iso_8601)] to the crate attributes to enable
[01:08:02] 
[01:08:02] thread 'time/mod.rs - time::SystemTime::display_iso_8601 (line 386)' panicked at 'couldn't compile the test', librustdoc/test.rs:333:13
[01:08:02] 
[01:08:02] 
[01:08:02] ---- time/mod.rs - time::SystemTimeDisplayIso8601 (line 450) stdout ----
[01:08:02] error[E0658]: use of unstable library feature 'system_time_display_iso_8601' (see issue #53891)
[01:08:02]  --> time/mod.rs:454:42
[01:08:02]   |
[01:08:02] 7 |     format!("{}", SystemTime::UNIX_EPOCH.display_iso_8601()));
[01:08:02]   |
[01:08:02]   |
[01:08:02]   = help: add #![feature(system_time_display_iso_8601)] to the crate attributes to enable
[01:08:02] 
[01:08:02] error[E0658]: use of unstable library feature 'system_time_display_iso_8601' (see issue #53891)
[01:08:02]  --> time/mod.rs:456:45
[01:08:02]   |
[01:08:02] 9 |     format!("{:.3}", SystemTime::UNIX_EPOCH.display_iso_8601()));
[01:08:02]   |
[01:08:02]   |
[01:08:02]   = help: add #![feature(system_time_display_iso_8601)] to the crate attributes to enable
[01:08:02] 
[01:08:02] error[E0658]: use of unstable library feature 'system_time_display_iso_8601' (see issue #53891)
[01:08:02]   --> time/mod.rs:458:45
[01:08:02]    |
[01:08:02] 11 |     format!("{:.0}", SystemTime::UNIX_EPOCH.display_iso_8601()));
[01:08:02]    |
[01:08:02]    |
[01:08:02]    = help: add #![feature(system_time_display_iso_8601)] to the crate attributes to enable
[01:08:02] 
[01:08:02] thread 'time/mod.rs - time::SystemTimeDisplayIso8601 (line 450)' panicked at 'couldn't compile the test', lib
