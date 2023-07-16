
2020-01-14T21:53:12.3332167Z error: the feature `slice_from_raw_parts` has been stable since 1.42.0 and no longer requires an attribute to enable
2020-01-14T21:53:12.3333402Z --> src/liballoc/lib.rs:107:12
2020-01-14T21:53:12.3333973Z |
2020-01-14T21:53:12.3334715Z 107 | #![feature(slice_from_raw_parts)]
2020-01-14T21:53:12.3335426Z | ^^^^^^^^^^^^^^^^^^^^
2020-01-14T21:53:12.3335944Z |
2020-01-14T21:53:12.3336914Z = note: `-D stable-features` implied by `-D warnings`
2020-01-14T21:53:12.3337186Z
2020-01-14T21:53:12.3957867Z error: aborting due to previous error
2020-01-14T21:53:12.3958825Z
2020-01-14T21:53:12.4101399Z error: could not compile `alloc`.
2020-01-14T21:53:12.4102638Z
2020-01-14T21:53:12.4103349Z To learn more, run the command again with --verbose.
2020-01-14T21:53:12.4176865Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-01-14T21:53:12.4177172Z Build completed unsuccessfully in 0:02:23
2020-01-14T21:53:12.4178280Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-14T21:53:12.4178532Z expected success, got: exit code: 101
2020-01-14T21:53:12.4192562Z == clock drift check ==
2020-01-14T21:53:12.4201121Z local time: Tue Jan 14 21:53:12 UTC 2020
2020-01-14T21:53:12.6971069Z network time: Tue, 14 Jan 2020 21:53:12 GMT
2020-01-14T21:53:12.6975506Z == end clock drift check == 
