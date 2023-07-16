rust
2019-11-11T17:11:57.1930360Z `vec![Bomb(0), Bomb(1)]` does.
2019-11-11T17:11:57.1930502Z fn main() {
2019-11-11T17:11:57.1930502Z fn main() {
2019-11-11T17:11:57.1930579Z makes its semantics identical to dropping the `[vec.len() - N..]`
2019-11-11T17:11:57.1930683Z panics printing `1` today and succeeds. With this change, it panics
2019-11-11T17:11:57.1930777Z printing `0` first (due to the drop order change), and then aborts
2019-11-11T17:11:57.1930956Z sub-slice tail of the vector, which is the same behavior as dropping a
2019-11-11T17:11:57.1931173Z vector containing the same sub-slice.
2019-11-11T17:11:57.1931173Z vector containing the same sub-slice.
2019-11-11T17:11:57.1931264Z with a double-panic printing `1`, just like dropping the
2019-11-11T17:11:57.1931417Z 
2019-11-11T17:11:57.1931477Z disk usage:
2019-11-11T17:11:57.2894098Z Filesystem            Size  Used Avail Use% Mounted on
2019-11-11T17:11:57.2894223Z C:/Program Files/Git  256G  140G  116G  55% /
---
2019-11-11T19:18:26.5253293Z [RUSTC-TIMING] vec_map test:false 0.304
2019-11-11T19:18:26.5309990Z    Compiling shell-escape v0.1.4
2019-11-11T19:18:27.4951357Z [RUSTC-TIMING] shell_escape test:false 0.941
2019-11-11T19:18:27.4999543Z    Compiling hex v0.4.0
2019-11-11T19:18:30.8159555Z memory allocation of 4294967304 bytes failed[RUSTC-TIMING] hex test:false 5.070
2019-11-11T19:18:30.8186277Z error: could not compile `hex`.
2019-11-11T19:18:30.8186665Z warning: build failed, waiting for other jobs to finish...
2019-11-11T19:18:30.8471437Z memory allocation of 536870920 bytes failed[RUSTC-TIMING] hex test:false 3.329
2019-11-11T19:18:30.8610886Z error: could not compile `hex`.
2019-11-11T19:18:30.8611578Z To learn more, run the command again with --verbose.
2019-11-11T19:18:30.8661589Z command did not execute successfully: "D:\\a\\1\\s\\build\\x86_64-pc-windows-gnu\\stage0\\bin\\cargo.exe" "build" "-Zconfig-profile" "--target" "x86_64-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--manifest-path" "D:\\a\\1\\s\\src/tools/cargo\\Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--message-format" "json-render-diagnostics"
2019-11-11T19:18:30.8661864Z expected success, got: exit code: 101
2019-11-11T19:18:30.9567202Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap dist
2019-11-11T19:18:30.9567202Z failed to run: D:\a\1\s\build\bootstrap\debug\bootstrap dist
2019-11-11T19:18:30.9567624Z Build completed unsuccessfully in 1:56:38
2019-11-11T19:18:31.0064759Z == clock drift check ==
2019-11-11T19:18:31.1001283Z   local time: Mon Nov 11 19:18:31 CUT 2019
2019-11-11T19:18:31.7346402Z   network time: Mon, 11 Nov 2019 19:18:31 GMT
2019-11-11T19:18:31.7365326Z == end clock drift check ==
2019-11-11T19:18:31.8075886Z 
2019-11-11T19:18:32.1761195Z ##[error]Bash exited with code '1'.
2019-11-11T19:18:32.2460027Z ##[section]Starting: Checkout
2019-11-11T19:18:32.3121061Z ==============================================================================
2019-11-11T19:18:32.3121187Z Task         : Get sources
2019-11-11T19:18:32.3121333Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
