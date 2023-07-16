`
T14:56:48.1468797Z [0m[0m[1m[32m  Downloaded[0m semver v0.9.0
2020-09-08T14:56:48.1574092Z [0m[0m[1m[32m  Downloaded[0m tracing-subscriber v0.2.11
2020-09-08T14:56:48.1937434Z [0m[0m[1m[32m  Downloaded[0m smallvec v0.6.13
2020-09-08T14:56:48.2043999Z [0m[0m[1m[33mwarning[0m[1m:[0m spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/itertools/0.8.2/download`, got 503
2020-09-08T14:56:48.2045897Z [0m[0m[1m[32m  Downloaded[0m tracing-tree v0.1.5
2020-09-08T14:56:48.2146432Z [0m[0m[1m[33mwarning[0m[1m:[0m spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/measureme/0.7.1/download`, got 503
2020-09-08T14:56:48.2149029Z [0m[0m[1m[32m  Downloaded[0m unicode-script v0.5.2
2020-09-08T14:56:48.2343641Z [0m[0m[1m[31merror[0m[1m:[0m failed to download from `https://crates.io/api/v1/crates/itertools/0.8.2/download`
2020-09-08T14:56:48.2343994Z 
2020-09-08T14:56:48.2344157Z Caused by:
2020-09-08T14:56:48.2344363Z   failed to get 200 response from `https://crates.io/api/v1/crates/itertools/0.8.2/download`, got 503
2020-09-08T14:56:48.2416120Z command did not execute successfully: "D:\\a\\rust\\rust\\build\\i686-pc-windows-gnu\\stage0\\bin\\cargo.exe" "build" "--target" "i686-pc-windows-gnu" "-Zbinary-dep-depinfo" "-j" "8" "--release" "--locked" "--color" "always" "--features" " llvm" "--manifest-path" "D:\\a\\rust\\rust\\compiler/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-09-08T14:56:48.2416465Z expected success, got: exit code:
