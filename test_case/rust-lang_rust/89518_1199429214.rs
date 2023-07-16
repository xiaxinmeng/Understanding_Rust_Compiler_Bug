plain
   Compiling addr2line v0.16.0
error[E0412]: cannot find type `c_int` in this scope
   --> library/std/src/sys/unix/fd.rs:167:52
    |
167 |                 cmp::min(bufs.len(), max_iov()) as c_int,
    |
help: consider importing one of these items
    |
6   | use core::ffi::c_int;
---

error[E0412]: cannot find type `c_int` in this scope
   --> library/std/src/sys/unix/fd.rs:255:52
    |
255 |                 cmp::min(bufs.len(), max_iov()) as c_int,
    |
help: consider importing one of these items
    |
6   | use core::ffi::c_int;
---
    |
153 |         target_os = "darwin",
    |         ^^^^^^^^^^^^^^^^^^^^
    |
    = note: `-D unexpected-cfgs` implied by `-D warnings`
    = note: expected values for `target_os` are: android, cuda, dragonfly, emscripten, espidf, freebsd, fuchsia, haiku, hermit, horizon, illumos, ios, l4re, linux, macos, netbsd, none, openbsd, psp, redox, solaris, solid_asp3, tvos, uefi, unknown, vxworks, wasi, watchos, windows, xous
error: unexpected `cfg` condition value
   --> library/std/src/sys/unix/fd.rs:176:9
    |
176 |         target_os = "darwin",
176 |         target_os = "darwin",
    |         ^^^^^^^^^^^^^^^^^^^^
    |
    = note: expected values for `target_os` are: android, cuda, dragonfly, emscripten, espidf, freebsd, fuchsia, haiku, hermit, horizon, illumos, ios, l4re, linux, macos, netbsd, none, openbsd, psp, redox, solaris, solid_asp3, tvos, uefi, unknown, vxworks, wasi, watchos, windows, xous
error: unexpected `cfg` condition value
   --> library/std/src/sys/unix/fd.rs:241:9
    |
241 |         target_os = "darwin",
241 |         target_os = "darwin",
    |         ^^^^^^^^^^^^^^^^^^^^
    |
    = note: expected values for `target_os` are: android, cuda, dragonfly, emscripten, espidf, freebsd, fuchsia, haiku, hermit, horizon, illumos, ios, l4re, linux, macos, netbsd, none, openbsd, psp, redox, solaris, solid_asp3, tvos, uefi, unknown, vxworks, wasi, watchos, windows, xous
error: unexpected `cfg` condition value
   --> library/std/src/sys/unix/fd.rs:264:9
    |
264 |         target_os = "darwin",
264 |         target_os = "darwin",
    |         ^^^^^^^^^^^^^^^^^^^^
    |
    = note: expected values for `target_os` are: android, cuda, dragonfly, emscripten, espidf, freebsd, fuchsia, haiku, hermit, horizon, illumos, ios, l4re, linux, macos, netbsd, none, openbsd, psp, redox, solaris, solid_asp3, tvos, uefi, unknown, vxworks, wasi, watchos, windows, xous
For more information about this error, try `rustc --explain E0412`.
error: could not compile `std` due to 6 previous errors
Build completed unsuccessfully in 0:00:23
