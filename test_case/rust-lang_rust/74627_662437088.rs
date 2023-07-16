plain
   Compiling pulldown-cmark v0.7.1
   Compiling cargo_metadata v0.9.1
   Compiling clippy v0.0.212 (/checkout/src/tools/clippy)
   Compiling clippy_lints v0.0.212 (/checkout/src/tools/clippy/clippy_lints)
error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
    --> src/tools/clippy/clippy_lints/src/utils/ast_utils.rs:509:14
     |
509  |             (DocComment(l), DocComment(r)) => l == r,
     |              ^^^^^^^^^^^^^ expected 2 fields, found 1

error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
    --> src/tools/clippy/clippy_lints/src/utils/ast_utils.rs:509:29
     |
509  |             (DocComment(l), DocComment(r)) => l == r,
     |                             ^^^^^^^^^^^^^ expected 2 fields, found 1

error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
    --> src/tools/clippy/clippy_lints/src/doc.rs:321:16
     |
321  |         if let AttrKind::DocComment(ref comment) = attr.kind {
     |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 fields, found 1

error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
    --> src/tools/clippy/clippy_lints/src/tabs_in_doc_comments.rs:63:16
     |
63   |         if let ast::AttrKind::DocComment(comment) = attr.kind {
     |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 fields, found 1
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0023`.
For more information about this error, try `rustc --explain E0023`.
error: could not compile `clippy_lints`.
To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/clippy/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
Building stage1 tool rls (x86_64-unknown-linux-gnu)
---
   Compiling tokio-threadpool v0.1.17
   Compiling tokio-timer v0.2.12
   Compiling tokio-current-thread v0.1.6
   Compiling tokio-codec v0.1.1
error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
    --> src/tools/clippy/clippy_lints/src/utils/ast_utils.rs:509:14
     |
509  |             (DocComment(l), DocComment(r)) => l == r,
     |              ^^^^^^^^^^^^^ expected 2 fields, found 1

error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
    --> src/tools/clippy/clippy_lints/src/utils/ast_utils.rs:509:29
     |
509  |             (DocComment(l), DocComment(r)) => l == r,
     |                             ^^^^^^^^^^^^^ expected 2 fields, found 1
   Compiling tokio-reactor v0.1.11
   Compiling tokio-reactor v0.1.11
error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
    --> src/tools/clippy/clippy_lints/src/doc.rs:321:16
     |
321  |         if let AttrKind::DocComment(ref comment) = attr.kind {
     |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 fields, found 1
   Compiling jsonrpc-pubsub v14.0.6
   Compiling miow v0.3.3
   Compiling tokio-fs v0.1.6
   Compiling tokio-udp v0.1.5
   Compiling tokio-udp v0.1.5
   Compiling tokio-uds v0.2.5
   Compiling tokio-tcp v0.1.3
error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
    --> src/tools/clippy/clippy_lints/src/tabs_in_doc_comments.rs:63:16
     |
63   |         if let ast::AttrKind::DocComment(comment) = attr.kind {
     |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 fields, found 1
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0023`.
For more information about this error, try `rustc --explain E0023`.
error: could not compile `clippy_lints`.
To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rls/Cargo.toml" "--features" "clippy, rustc-workspace-hack/all-static" "--message-format" "json-render-diagnostics"
---
Dist rust-analyzer stage1 (x86_64-unknown-linux-gnu)
 finished in 12.397
Dist LlvmTools (x86_64-unknown-linux-gnu)
 finished in 20.165
thread 'main' panicked at 'clippy expected to build - essential tool', src/bootstrap/dist.rs:1479:14
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
Build completed unsuccessfully in 0:32:53
== clock drift check ==
  local time: Wed Jul 22 12:55:22 UTC 2020
  local time: Wed Jul 22 12:55:22 UTC 2020
  network time: Wed, 22 Jul 2020 12:55:22 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (6220) (python)
