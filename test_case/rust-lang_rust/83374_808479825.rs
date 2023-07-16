plain
    Checking hashbrown v0.11.0
    Checking object v0.22.0
    Checking miniz_oxide v0.4.0
    Checking addr2line v0.14.0
error[E0308]: `if` and `else` have incompatible types
    |
    |
438 |               let cmsg = if let Some(current) = self.current {
    |  ________________________-
439 | |                 libc::CMSG_NXTHDR(&msg, current)
440 | |             } else {
440 | |             } else {
441 | |                 libc::CMSG_FIRSTHDR(&msg);
    | |                 |                        |
    | |                 |                        help: consider removing this semicolon
    | |                 |                        help: consider removing this semicolon
    | |                 expected *-ptr, found `()`
442 | |             };
    | |_____________- `if` and `else` have incompatible types
    |
    = note:   expected type `*mut cmsghdr`

error[E0308]: mismatched types
   --> library/std/src/sys/unix/ext/net/ancillary.rs:455:33
    |
    |
455 |             self.current = Some(cmsg);
    |                                 ^^^^ expected `&cmsghdr`, found *-ptr
    |
    = note: expected reference `&cmsghdr`
             found raw pointer `*const cmsghdr`
error[E0308]: mismatched types
   --> library/std/src/sys/unix/ext/net/ancillary.rs:456:68
    |
    |
456 |             let ancillary_result = AncillaryData::try_from_cmsghdr(cmsg);
    |                                                                    ^^^^ expected `&cmsghdr`, found *-ptr
    |
    = note: expected reference `&cmsghdr`
             found raw pointer `*const cmsghdr`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0308`.
error: could not compile `std`
error: could not compile `std`

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:46
