plain
[00:32:41]    Compiling bitflags v0.9.1
[00:32:44]    Compiling rand v0.4.2
[00:32:49]    Compiling tempdir v0.3.7
[00:32:49]    Compiling rustdoc v0.0.0 (file:///checkout/src/librustdoc)
[00:33:04] error[E0599]: no method named `is_whitespace` found for type `&str` in the current scope
[00:33:04]     |
[00:33:04]     |
[00:33:04] 440 |         let header = trimline.is_whitespace() ||
[00:33:04]     |
[00:33:04]     |
[00:33:04]     = help: items from traits can only be used if the trait is in scope
[00:33:04]     = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:33:04]             candidate #1: `use core::str::StrExt;`
[00:33:04]     = help: did you mean `split_whitespace`?
[00:33:04] error: aborting due to previous error
[00:33:04] 
[00:33:04] For more information about this error, try `rustc --explain E0599`.
[00:33:04] error: Could not compile `rustdoc`.
---
[00:33:04] 
[00:33:04] 
[00:33:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:33:04] Build completed unsuccessfully in 0:28:30
[00:33:04] make: *** [all] Error 1
[00:33:04] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1397eb12
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
