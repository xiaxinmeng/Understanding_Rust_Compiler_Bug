plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
    Checking unicode-segmentation v1.10.0
    Checking globset v0.4.9
    Checking annotate-snippets v0.9.1
    Checking ignore v0.4.18
error[E0004]: non-exhaustive patterns: `TyKind::AnonEnum(_)` not covered
     |
664  |         match self.kind {
664  |         match self.kind {
     |               ^^^^^^^^^ pattern `TyKind::AnonEnum(_)` not covered
note: `TyKind` defined here
    --> /checkout/compiler/rustc_ast/src/ast.rs:2101:5
     |
2054 | pub enum TyKind {
2054 | pub enum TyKind {
     | ---------------
...
2101 |     AnonEnum(Vec<P<Ty>>),
     = note: the matched value is of type `TyKind`
     = note: the matched value is of type `TyKind`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
849  ~             ),
849  ~             ),
850  ~             TyKind::AnonEnum(_) => todo!(),

For more information about this error, try `rustc --explain E0004`.
error: could not compile `rustfmt-nightly` due to previous error
warning: build failed, waiting for other jobs to finish...
