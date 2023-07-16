plain
....................i.....................i.....................i.....................i............. 3300/3378
............................F.................................................
failures:

---- src/time.rs - time (line 9) stdout ----
error[E0599]: no function or associated item named `from_seconds` found for struct `Duration` in the current scope
   |
   |
10 | let ten_seconds = Duration::from_seconds(10);
   |                             |
   |                             function or associated item not found in `Duration`
   |                             help: there is an associated function with a similar name: `from_secs`

---

error: test failed, to rerun pass '--doc'


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "-p" "core" "--" "--quiet"


Build completed unsuccessfully in 0:19:23
