plain
   Compiling structopt-derive v0.4.9
    Checking thiserror v1.0.20
    Checking structopt v0.3.16
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
error[E0004]: non-exhaustive patterns: `AnonymousStruct(_, _)` and `AnonymousUnion(_, _)` not covered
     |
629  |         match self.kind {
629  |         match self.kind {
     |               ^^^^^^^^^ patterns `AnonymousStruct(_, _)` and `AnonymousUnion(_, _)` not covered
    ::: /checkout/compiler/rustc_ast/src/ast.rs:1865:5
     |
     |
1865 |     AnonymousStruct(Vec<FieldDef>, bool),
     |     --------------- not covered
1866 |     /// An anonymous union type i.e. `union { bar: Type }`
1867 |     AnonymousUnion(Vec<FieldDef>, bool),
     |
     = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
     = note: the matched value is of type `TyKind`

