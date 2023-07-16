plain
    Checking addr2line v0.14.0
error: unreachable pattern
   --> library/std/src/io/error.rs:306:13
    |
305 |             NoStorageSpace => "no storage space",
    |             -------------- matches any value
306 |             NotConnected => "not connected",
    |             ^^^^^^^^^^^^ unreachable pattern
    |
    = note: `-D unreachable-patterns` implied by `-D warnings`
error: unreachable pattern
   --> library/std/src/io/error.rs:307:13
    |
    |
305 |             NoStorageSpace => "no storage space",
    |             -------------- matches any value
306 |             NotConnected => "not connected",
307 |             NotFound => "entity not found",
    |             ^^^^^^^^ unreachable pattern
error: unreachable pattern
   --> library/std/src/io/error.rs:308:13
    |
    |
305 |             NoStorageSpace => "no storage space",
    |             -------------- matches any value
...
308 |             NotSupported => "not supported",
    |             ^^^^^^^^^^^^ unreachable pattern
error: unreachable pattern
   --> library/std/src/io/error.rs:309:13
    |
    |
305 |             NoStorageSpace => "no storage space",
    |             -------------- matches any value
...
309 |             Other => "other os error",
    |             ^^^^^ unreachable pattern
error: unreachable pattern
   --> library/std/src/io/error.rs:310:13
    |
    |
305 |             NoStorageSpace => "no storage space",
    |             -------------- matches any value
...
310 |             PermissionDenied => "permission denied",
    |             ^^^^^^^^^^^^^^^^ unreachable pattern
error: unreachable pattern
   --> library/std/src/io/error.rs:311:13
    |
    |
305 |             NoStorageSpace => "no storage space",
    |             -------------- matches any value
...
311 |             ReadOnlyFilesystem => "read-only filesystem or storage medium",
    |             ^^^^^^^^^^^^^^^^^^ unreachable pattern
error: unreachable pattern
   --> library/std/src/io/error.rs:312:13
    |
    |
305 |             NoStorageSpace => "no storage space",
    |             -------------- matches any value
...
312 |             StaleNetworkFileHandle => "stale network file handle",
    |             ^^^^^^^^^^^^^^^^^^^^^^ unreachable pattern
error: unreachable pattern
   --> library/std/src/io/error.rs:313:13
    |
    |
305 |             NoStorageSpace => "no storage space",
    |             -------------- matches any value
...
313 |             SymbolicLinkLoop => "symbolic link loop",
    |             ^^^^^^^^^^^^^^^^ unreachable pattern
error: unreachable pattern
   --> library/std/src/io/error.rs:314:13
    |
    |
305 |             NoStorageSpace => "no storage space",
    |             -------------- matches any value
...
314 |             InvalidSeek => "invalid seek",
    |             ^^^^^^^^^^^ unreachable pattern
error: unreachable pattern
   --> library/std/src/io/error.rs:315:13
    |
    |
305 |             NoStorageSpace => "no storage space",
    |             -------------- matches any value
...
315 |             TimedOut => "timed out",
    |             ^^^^^^^^ unreachable pattern
error: unreachable pattern
   --> library/std/src/io/error.rs:316:13
    |
    |
305 |             NoStorageSpace => "no storage space",
    |             -------------- matches any value
...
316 |             TooManyLinks => "too many links",
    |             ^^^^^^^^^^^^ unreachable pattern
error: unreachable pattern
   --> library/std/src/io/error.rs:317:13
    |
    |
305 |             NoStorageSpace => "no storage space",
    |             -------------- matches any value
...
317 |             UnexpectedEof => "unexpected end of file",
    |             ^^^^^^^^^^^^^ unreachable pattern
error: unreachable pattern
   --> library/std/src/io/error.rs:318:13
    |
    |
305 |             NoStorageSpace => "no storage space",
    |             -------------- matches any value
...
318 |             WouldBlock => "operation would block",
    |             ^^^^^^^^^^ unreachable pattern
error: unreachable pattern
   --> library/std/src/io/error.rs:319:13
    |
    |
305 |             NoStorageSpace => "no storage space",
    |             -------------- matches any value
...
319 |             WriteZero => "write zero",
    |             ^^^^^^^^^ unreachable pattern

error: unused variable: `NoStorageSpace`
    |
    |
305 |             NoStorageSpace => "no storage space",
    |             ^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_NoStorageSpace`
    |
    = note: `-D unused-variables` implied by `-D warnings`

error: unused variable: `InvalidSeek`
    |
    |
314 |             InvalidSeek => "invalid seek",
    |             ^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_InvalidSeek`
error: aborting due to 16 previous errors

error: could not compile `std`


To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:35
