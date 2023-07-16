
[00:06:44] error[E0252]: the name `DiagnosticBuilder` is defined multiple times
[00:06:44]   --> /checkout/src/librustc/ty/maps.rs:13:26
[00:06:44]    |
[00:06:44] 11 | use errors::DiagnosticBuilder;
[00:06:44]    |     ------------------------- previous import of the type `DiagnosticBuilder` here
[00:06:44] 12 | use dep_graph::{DepConstructor, DepNode, DepNodeIndex};
[00:06:44] 13 | use errors::{Diagnostic, DiagnosticBuilder};
[00:06:44]    |                          ^^^^^^^^^^^^^^^^^ `DiagnosticBuilder` reimported here
