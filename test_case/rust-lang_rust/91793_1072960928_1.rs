ignore")]
    |                                                                    ^^^^^^^^^ help: did you mean: `freebsd12`
error: unexpected `cfg` condition name
   --> library/std/src/os/unix/net/datagram.rs:868:64
    |
    |
868 |     #[cfg(any(doc, target_os = "android", target_os = "linux", freebsd13))]
    |                                                                ^^^^^^^^^ help: did you mean: `freebsd12`
error: unexpected `cfg` condition name
   --> library/std/src/os/unix/net/datagram.rs:880:64
    |
    |
880 |     #[cfg(any(doc, target_os = "android", target_os = "linux", freebsd13))]
    |                                                                ^^^^^^^^^ help: did you mean: `freebsd12`
error: unexpected `cfg` condition name
   --> library/std/src/os/unix/net/stream.rs:408:82
    |
    |
408 |         any(target_os = "android", target_os = "linux", target_os = "dragonfly", freebsd13),
    |                                                                                  ^^^^^^^^^ help: did you mean: `freebsd12`
error: unexpected `cfg` condition name
   --> library/std/src/os/unix/net/stream.rs:412:86
    |
    |
412 |         not(any(target_os = "android", target_os = "linux", target_os = "dragonfly", freebsd13)),
    |                                                                                      ^^^^^^^^^ help: did you mean: `freebsd12`
error: unexpected `cfg` condition name
   --> library/std/src/os/unix/net/stream.rs:429:9
    |
429 |         freebsd13
429 |         freebsd13
    |         ^^^^^^^^^ help: did you mean: `freebsd12`
error: unexpected `cfg` condition name
   --> library/std/src/os/unix/net/stream.rs:447:9
    |
447 |         freebsd13
447 |         freebsd13
    |         ^^^^^^^^^ help: did you mean: `freebsd12`
error: unexpected `cfg` condition name
   --> library/std/src/sys/unix/net.rs:422:11
    |
422 |     #[cfg(freebsd13)]
422 |     #[cfg(freebsd13)]
    |           ^^^^^^^^^ help: did you mean: `freebsd12`
error: unexpected `cfg` condition name
   --> library/std/src/sys/unix/net.rs:427:11
    |
427 |     #[cfg(freebsd13)]
427 |     #[cfg(freebsd13)]
    |           ^^^^^^^^^ help: did you mean: `freebsd12`
error: could not compile `std` due to 22 previous errors
Build completed unsuccessfully in 0:03:47
