
 Downloading crates ...
warning: spurious network error (2 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/mac/0.1.1/download`, got 502
warning: spurious network error (1 tries remaining): failed to get 200 response from `https://crates.io/api/v1/crates/mac/0.1.1/download`, got 502
error: failed to download from `https://crates.io/api/v1/crates/mac/0.1.1/download`

Caused by:
  failed to get 200 response from `https://crates.io/api/v1/crates/mac/0.1.1/download`, got 502
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustbook/Cargo.toml" "--message-format" "json-render-diagnostics"
