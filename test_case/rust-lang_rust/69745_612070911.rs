
Dist extended stage1 (x86_64-apple-darwin)
Building stage1 tool cargo (x86_64-apple-darwin)
dyld: Library not loaded: /usr/lib/libcurl.4.dylib
  Referenced from: /Users/runner/runners/2.165.2/work/1/s/build/x86_64-apple-darwin/stage0/bin/cargo
  Reason: image not found
command did not execute successfully: "/Users/runner/runners/2.165.2/work/1/s/build/x86_64-apple-darwin/stage0/bin/cargo" "build" "--target" "x86_64-apple-darwin" "-Zbinary-dep-depinfo" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/Users/runner/runners/2.165.2/work/1/s/src/tools/cargo/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--message-format" "json-render-diagnostics"
expected success, got: signal: 6
failed to run: /Users/runner/runners/2.165.2/work/1/s/build/bootstrap/debug/bootstrap dist
Build completed unsuccessfully in 1:11:06
