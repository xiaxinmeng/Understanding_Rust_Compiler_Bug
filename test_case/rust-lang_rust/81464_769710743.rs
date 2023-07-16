plain
[RUSTC-TIMING] addr2line test:false 0.452
[RUSTC-TIMING] core test:false 41.004
[RUSTC-TIMING] gimli test:false 5.931
[RUSTC-TIMING] object test:false 10.861
error: use of deprecated type alias `libc::time_t`: This type is changed to 64-bit in musl 1.2.0, we'll follow that change in the future release. See #1848 for more info.
   |
   |
15 |     libc::timespec { tv_sec: <libc::time_t>::MAX, tv_nsec: 1_000_000_000 - 1 };
   |
   |
   = note: `-D deprecated` implied by `-D warnings`

error: use of deprecated type alias `libc::time_t`: This type is changed to 64-bit in musl 1.2.0, we'll follow that change in the future release. See #1848 for more info.
   |
   |
17 | fn saturating_cast_to_time_t(value: u64) -> libc::time_t {


error: use of deprecated type alias `libc::time_t`: This type is changed to 64-bit in musl 1.2.0, we'll follow that change in the future release. See #1848 for more info.
   |
   |
18 |     if value > <libc::time_t>::MAX as u64 { <libc::time_t>::MAX } else { value as libc::time_t }


error: use of deprecated type alias `libc::time_t`: This type is changed to 64-bit in musl 1.2.0, we'll follow that change in the future release. See #1848 for more info.
   |
   |
18 |     if value > <libc::time_t>::MAX as u64 { <libc::time_t>::MAX } else { value as libc::time_t }


error: use of deprecated type alias `libc::time_t`: This type is changed to 64-bit in musl 1.2.0, we'll follow that change in the future release. See #1848 for more info.
   |
   |
18 |     if value > <libc::time_t>::MAX as u64 { <libc::time_t>::MAX } else { value as libc::time_t }


error: use of deprecated type alias `libc::time_t`: This type is changed to 64-bit in musl 1.2.0, we'll follow that change in the future release. See #1848 for more info.
   |
   |
91 |             .checked_add((nsec / 1_000_000_000) as libc::time_t)


error: use of deprecated type alias `libc::time_t`: This type is changed to 64-bit in musl 1.2.0, we'll follow that change in the future release. See #1848 for more info.
    |
    |
303 |             tv_sec: self.stat.st_mtime as libc::time_t,


error: use of deprecated type alias `libc::time_t`: This type is changed to 64-bit in musl 1.2.0, we'll follow that change in the future release. See #1848 for more info.
    |
    |
319 |             tv_sec: self.stat.st_atime as libc::time_t,


error: use of deprecated type alias `libc::time_t`: This type is changed to 64-bit in musl 1.2.0, we'll follow that change in the future release. See #1848 for more info.
   --> library/std/src/sys/unix/net.rs:333:47
    |
333 |                 let secs = if dur.as_secs() > libc::time_t::MAX as u64 {


error: use of deprecated type alias `libc::time_t`: This type is changed to 64-bit in musl 1.2.0, we'll follow that change in the future release. See #1848 for more info.
   --> library/std/src/sys/unix/net.rs:334:21
334 |                     libc::time_t::MAX
    |                     ^^^^^^^^^^^^^^^^^


error: use of deprecated type alias `libc::time_t`: This type is changed to 64-bit in musl 1.2.0, we'll follow that change in the future release. See #1848 for more info.
   --> library/std/src/sys/unix/net.rs:336:38
    |
336 |                     dur.as_secs() as libc::time_t


error: use of deprecated type alias `libc::time_t`: This type is changed to 64-bit in musl 1.2.0, we'll follow that change in the future release. See #1848 for more info.
   --> library/std/src/sys/unix/thread.rs:159:38
    |
159 |                     tv_sec: cmp::min(libc::time_t::MAX as u64, secs) as libc::time_t,


error: use of deprecated type alias `libc::time_t`: This type is changed to 64-bit in musl 1.2.0, we'll follow that change in the future release. See #1848 for more info.
   --> library/std/src/sys/unix/thread.rs:159:73
    |
159 |                     tv_sec: cmp::min(libc::time_t::MAX as u64, secs) as libc::time_t,

error: aborting due to 13 previous errors

[RUSTC-TIMING] std test:false 2.544
[RUSTC-TIMING] std test:false 2.544
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-musl/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-musl" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace profiler compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --build x86_64-unknown-linux-musl
Build completed unsuccessfully in 0:02:46
