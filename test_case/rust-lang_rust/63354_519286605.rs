plain
2019-08-07T20:53:52.0939776Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-07T20:53:52.0939846Z 
2019-08-07T20:53:52.0940069Z   git checkout -b <new-branch-name>
2019-08-07T20:53:52.0940146Z 
2019-08-07T20:53:52.0940442Z HEAD is now at 7f7fec7df Auto merge of #63354 - pietroalbini:rollup-gzp7eop, r=pietroalbini
2019-08-07T20:53:52.1102974Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-07T20:53:52.1105813Z ==============================================================================
2019-08-07T20:53:52.1105900Z Task         : Bash
2019-08-07T20:53:52.1106016Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-07T22:04:15.6667723Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "arm-linux-androideabi" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-08-07T22:04:15.6667965Z expected success, got: exit code: 101
2019-08-07T22:04:15.6679895Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --target arm-linux-androideabi,armv7-linux-androideabi,thumbv7neon-linux-androideabi,i686-linux-android,aarch64-linux-android,x86_64-linux-android
2019-08-07T22:04:15.6680572Z Build completed unsuccessfully in 1:02:15
2019-08-07T22:04:17.0122697Z ##[error]Bash exited with code '1'.
2019-08-07T22:04:17.0162192Z ##[section]Starting: Upload CPU usage statistics
2019-08-07T22:04:17.0171924Z ==============================================================================
2019-08-07T22:04:17.0172021Z Task         : Bash
2019-08-07T22:04:17.0172112Z Description  : Run a Bash script on macOS, Linux, or Windows
