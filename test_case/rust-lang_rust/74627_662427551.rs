plain
Initialized empty Git repository in /home/runner/work/rust/rust/.git/
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/74627/merge:refs/remotes/pull/74627/merge
---
    Checking url v2.1.0
    Checking toml v0.5.3
    Checking cargo_metadata v0.9.1
    Checking clippy_lints v0.0.212 (/checkout/src/tools/clippy/clippy_lints)
error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
    --> src/tools/clippy/clippy_lints/src/utils/ast_utils.rs:509:14
     |
509  |             (DocComment(l), DocComment(r)) => l == r,
     |              ^^^^^^^^^^^^^ expected 2 fields, found 1
    ::: /checkout/src/librustc_ast/ast.rs:2368:5
     |
     |
2368 |     DocComment(CommentKind, Symbol),
     |     ------------------------------- tuple variant defined here

error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
    --> src/tools/clippy/clippy_lints/src/utils/ast_utils.rs:509:29
     |
509  |             (DocComment(l), DocComment(r)) => l == r,
     |                             ^^^^^^^^^^^^^ expected 2 fields, found 1
    ::: /checkout/src/librustc_ast/ast.rs:2368:5
     |
     |
2368 |     DocComment(CommentKind, Symbol),
     |     ------------------------------- tuple variant defined here

error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
    --> src/tools/clippy/clippy_lints/src/doc.rs:321:16
     |
321  |         if let AttrKind::DocComment(ref comment) = attr.kind {
     |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 fields, found 1
    ::: /checkout/src/librustc_ast/ast.rs:2368:5
     |
     |
2368 |     DocComment(CommentKind, Symbol),
     |     ------------------------------- tuple variant defined here

error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
    --> src/tools/clippy/clippy_lints/src/tabs_in_doc_comments.rs:63:16
     |
63   |         if let ast::AttrKind::DocComment(comment) = attr.kind {
     |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 2 fields, found 1
    ::: /checkout/src/librustc_ast/ast.rs:2368:5
     |
     |
2368 |     DocComment(CommentKind, Symbol),
     |     ------------------------------- tuple variant defined here
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0023`.
For more information about this error, try `rustc --explain E0023`.
error: could not compile `clippy_lints`.
To learn more, run the command again with --verbose.
To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--manifest-path" "/checkout/src/tools/clippy/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:04:17
== clock drift check ==
  local time: Wed Jul 22 12:31:50 UTC 2020
  local time: Wed Jul 22 12:31:50 UTC 2020
  network time: Wed, 22 Jul 2020 12:31:50 GMT
== end clock drift check ==
##[error]Process completed with exit code 1.
Terminate orphan process: pid (6757) (python)
