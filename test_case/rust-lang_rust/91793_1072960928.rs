plain
   Compiling addr2line v0.16.0
error: unexpected `cfg` condition name
  --> library/std/src/os/unix/net/ancillary.rs:24:7
   |
24 | #[cfg(freebsd13)]
   |       ^^^^^^^^^ help: did you mean: `freebsd12`
   |
   = note: `-D unexpected-cfgs` implied by `-D warnings`
error: unexpected `cfg` condition name
  --> library/std/src/os/unix/net/ancillary.rs:38:7
   |
38 | #[cfg(freebsd13)]
38 | #[cfg(freebsd13)]
   |       ^^^^^^^^^ help: did you mean: `freebsd12`
error: unexpected `cfg` condition name
   --> library/std/src/os/unix/net/ancillary.rs:212:7
    |
212 | #[cfg(freebsd13)]
212 | #[cfg(freebsd13)]
    |       ^^^^^^^^^ help: did you mean: `freebsd12`
error: unexpected `cfg` condition name
   --> library/std/src/os/unix/net/ancillary.rs:268:7
    |
268 | #[cfg(freebsd13)]
268 | #[cfg(freebsd13)]
    |       ^^^^^^^^^ help: did you mean: `freebsd12`
error: unexpected `cfg` condition name
   --> library/std/src/os/unix/net/ancillary.rs:350:7
    |
350 | #[cfg(freebsd13)]
350 | #[cfg(freebsd13)]
    |       ^^^^^^^^^ help: did you mean: `freebsd12`
error: unexpected `cfg` condition name
   --> library/std/src/os/unix/net/ancillary.rs:354:60
    |
    |
354 | #[cfg(any(doc, target_os = "android", target_os = "linux", freebsd13))]
    |                                                            ^^^^^^^^^ help: did you mean: `freebsd12`
error: unexpected `cfg` condition name
   --> library/std/src/os/unix/net/ancillary.rs:109:15
    |
    |
109 |     #[cfg(not(freebsd13))]
    |               ^^^^^^^^^ help: did you mean: `freebsd12`
error: unexpected `cfg` condition name
   --> library/std/src/os/unix/net/ancillary.rs:111:11
    |
111 |     #[cfg(freebsd13)]
111 |     #[cfg(freebsd13)]
    |           ^^^^^^^^^ help: did you mean: `freebsd12`
error: unexpected `cfg` condition name
   --> library/std/src/os/unix/net/ancillary.rs:381:9
    |
381 |         freebsd13
381 |         freebsd13
    |         ^^^^^^^^^ help: did you mean: `freebsd12`
error: unexpected `cfg` condition name
   --> library/std/src/os/unix/net/ancillary.rs:405:64
    |
    |
405 |     #[cfg(any(doc, target_os = "android", target_os = "linux", freebsd13))]
    |                                                                ^^^^^^^^^ help: did you mean: `freebsd12`
error: unexpected `cfg` condition name
   --> library/std/src/os/unix/net/ancillary.rs:424:27
    |
424 |                     #[cfg(freebsd13)]
424 |                     #[cfg(freebsd13)]
    |                           ^^^^^^^^^ help: did you mean: `freebsd12`
error: unexpected `cfg` condition name
   --> library/std/src/os/unix/net/ancillary.rs:650:11
    |
650 |     #[cfg(freebsd13)]
650 |     #[cfg(freebsd13)]
    |           ^^^^^^^^^ help: did you mean: `freebsd12`
error: unexpected `cfg` condition name
   --> library/std/src/os/unix/net/datagram.rs:857:64
    |
    |
857 |     #[cfg_attr(any(target_os = "android", target_os = "linux", freebsd13,), doc = "