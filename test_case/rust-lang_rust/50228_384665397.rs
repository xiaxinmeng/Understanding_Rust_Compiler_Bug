plain
[00:47:18]    Compiling rustdoc v0.0.0 (file:///checkout/src/librustdoc)
[00:47:20] error[E0432]: unresolved import `syntax::abi`
[00:47:20]   --> librustdoc/html/render.rs:57:14
[00:47:20]    |
[00:47:20] 57 | use syntax::{abi, ast};
[00:47:20]    |              ^^^ no `abi` in the root
[00:47:20] 
[00:47:20] error[E0433]: failed to resolve. Maybe a missing `extern crate rustc_target;`?
[00:47:20]    |
[00:47:20] 68 | use rustc_target::spec::TargetTriple;
[00:47:20] 68 | use rustc_target::spec::TargetTriple;
[00:47:20]    |     ^^^^^^^^^^^^ Maybe a missing `extern crate rustc_target;`?
[00:47:20] 
[00:47:20] error[E0433]: failed to resolve. Maybe a missing `extern crate rustc_target;`?
[00:47:20]   --> librustdoc/clean/mod.rs:23:5
[00:47:20]    |
[00:47:20] 23 | use rustc_target::spec::abi::Abi;
[00:47:20]    |     ^^^^^^^^^^^^ Maybe a missing `extern crate rustc_target;`?
[00:47:20] 
[00:47:20] error[E0433]: failed to resolve. Maybe a missing `extern crate rustc_target;`?
[00:47:20]    |
[00:47:20] 26 | use rustc_target::spec::TargetTriple;
[00:47:20] 26 | use rustc_target::spec::TargetTriple;
[00:47:20]    |     ^^^^^^^^^^^^ Maybe a missing `extern crate rustc_target;`?
[00:47:20] 
[00:47:20] error[E0433]: failed to resolve. Maybe a missing `extern crate rustc_target;`?
[00:47:20]   --> librustdoc/doctree.rs:16:5
[00:47:20] 16 | use rustc_target::spec::abi;
[00:47:20] 16 | use rustc_target::spec::abi;
[00:47:20]    |     ^^^^^^^^^^^^ Maybe a missing `extern crate rustc_target;`?
[00:47:20] 
[00:47:20] error[E0433]: failed to resolve. Maybe a missing `extern crate rustc_target;`?
[00:47:20]   --> librustdoc/html/format.rs:22:5
[00:47:20]    |
[00:47:20] 22 | use rustc_target::spec::abi::Abi;
[00:47:20]    |     ^^^^^^^^^^^^ Maybe a missing `extern crate rustc_target;`?
[00:47:20] 
[00:47:20] error[E0433]: failed to resolve. Maybe a missing `extern crate rustc_target;`?
[00:47:20]   --> librustdoc/visit_ast.rs:16:5
[00:47:20] 16 | use rustc_target::spec::abi;
[00:47:20] 16 | use rustc_target::spec::abi;
[00:47:20]    |     ^^^^^^^^^^^^ Maybe a missing `extern crate rustc_target;`?
[00:47:20] 
[00:47:20] error[E0433]: failed to resolve. Use of undeclared type or module `Abi`
[00:47:20]     --> librustdoc/clean/mod.rs:3732:26
[00:47:20]      |
[00:47:20] 3732 |                     abi: Abi::Rust,
[00:47:20]      |                          ^^^ Use of undeclared type or module `Abi`
[00:47:20] 
[00:47:20] error[E0433]: failed to resolve. Use of undeclared type or module `TargetTriple`
[00:47:20]     |
[00:47:20]     |
[00:47:20] 142 |     let host_triple = TargetTriple::from_triple(config::host_triple());
[00:47:20]     |                       ^^^^^^^^^^^^ Use of undeclared type or module `TargetTriple`
[00:47:20] 
[00:47:20] error[E0433]: failed to resolve. Use of undeclared type or module `abi`
[00:47:20]    --> librustdoc/doctree.rs:156:14
[00:47:20]     |
[00:47:20] 156 |     pub abi: abi::Abi,
[00:47:20]     |              ^^^ Use of undeclared type or module `abi`
[00:47:20] 
[00:47:20] error[E0433]: failed to resolve. Use of undeclared type or module `Abi`
[00:47:20]     --> librustdoc/html/format.rs:1024:13
[00:47:20]      |
[00:47:20] 1024 |             Abi::Rust => Ok(()),
[00:47:20]      |             ^^^ Use of undeclared type or module `Abi`
[00:47:20] 
[00:47:20] error[E0433]: failed to resolve. Use of undeclared type or module `abi`
[00:47:20]    --> librustdoc/visit_ast.rs:179:27
[00:47:20]     |
[00:47:20] 179 |                     abi: &abi::Abi,
[00:47:20]     |                           ^^^ Use of undeclared type or module `abi`
[00:47:20] 
[00:47:20] error[E0433]: failed to resolve. Use of undeclared type or module `TargetTriple`
[00:47:20]     |
[00:47:20]     |
[00:47:20] 621 |             TargetTriple::TargetPath(PathBuf::from(target))
[00:47:20]     |             ^^^^^^^^^^^^ Use of undeclared type or module `TargetTriple`
[00:47:20] 
[00:47:20] error[E0433]: failed to resolve. Use of undeclared type or module `TargetTriple`
[00:47:20]     |
[00:47:20]     |
[00:47:20] 623 |             TargetTriple::TargetTriple(target)
[00:47:20]     |             ^^^^^^^^^^^^ Use of undeclared type or module `TargetTriple`
[00:47:20] 
[00:47:20] error[E0412]: cannot find type `Abi` in this scope
[00:47:20]     --> librustdoc/clean/mod.rs:1867:14
[00:47:20]      |
[00:47:20] 1867 |     pub abi: Abi,
[00:47:20]      |              ^^^ not found in this scope
[00:47:20] help: possible candidate is found in another module, you can import it into scope
[00:47:20]      |
[00:47:20] 14   | use rustc::ty::layout::Abi;
[00:47:20] 
[00:47:20] 
[00:47:20] error[E0412]: cannot find type `Abi` in this scope
[00:47:20]     --> librustdoc/clean/mod.rs:1888:14
[00:47:20]      |
[00:47:20] 1888 |     pub abi: Abi,
[00:47:20]      |              ^^^ not found in this scope
[00:47:20] help: possible candidate is found in another module, you can import it into scope
[00:47:20]      |
[00:47:20] 14   | use rustc::ty::layout::Abi;
[00:47:20] 
[00:47:20] 
[00:47:20] error[E0412]: cannot find type `Abi` in this scope
[00:47:20]     --> librustdoc/clean/mod.rs:1897:14
[00:47:20]      |
[00:47:20] 1897 |     pub abi: Abi,
[00:47:20]      |              ^^^ not found in this scope
[00:47:20] help: possible candidate is found in another module, you can import it into scope
[00:47:20]      |
[00:47:20] 14   | use rustc::ty::layout::Abi;
[00:47:20] 
[00:47:20] 
[00:47:20] error[E0412]: cannot find type `Abi` in this scope
[00:47:20]     --> librustdoc/clean/mod.rs:3413:14
[00:47:20]      |
[00:47:20] 3413 |     pub abi: Abi,
[00:47:20]      |              ^^^ not found in this scope
[00:47:20] help: possible candidate is found in another module, you can import it into scope
[00:47:20]      |
[00:47:20] 14   | use rustc::ty::layout::Abi;
[00:47:20] 
[00:47:20] 
[00:47:20] error[E0412]: cannot find type `TargetTriple` in this scope
[00:47:20]     |
[00:47:20]     |
[00:47:20] 124 |                 triple: Option<TargetTriple>,
[00:47:20] 
[00:47:20] 
[00:47:20] error[E0412]: cannot find type `Abi` in this scope
[00:47:20]   --> librustdoc/html/format.rs:52:25
[00:47:20]    |
[00:47:20] 52 | pub struct AbiSpace(pub Abi);
[00:47:20]    |                         ^^^ not found in this scope
[00:47:20] help: possible candidate is found in another module, you can import it into scope
[00:47:20]    |
[00:47:20] 18 | use rustc::ty::layout::Abi;
[00:47:20] 
[00:47:20] 
[00:47:21] error: unused import: `rustc_target::spec::TargetTriple`
[00:47:21]    |
[00:47:21] 68 | use rustc_target::spec::TargetTriple;
[00:47:21]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:47:21]    |
[00:47:21]    |
[00:47:21]    = note: `-D unused-imports` implied by `-D warnings`
[00:47:21] 
[00:47:21] error: unused import: `rustc_target::spec::abi::Abi`
[00:47:21]   --> librustdoc/clean/mod.rs:23:5
[00:47:21]    |
[00:47:21] 23 | use rustc_target::spec::abi::Abi;
[00:47:21] 
[00:47:21] 
[00:47:21] error: unused import: `rustc_target::spec::TargetTriple`
[00:47:21]    |
[00:47:21] 26 | use rustc_target::spec::TargetTriple;
[00:47:21]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:47:21] 
[00:47:21] 
[00:47:21] error: unused import: `rustc_target::spec::abi`
[00:47:21]   --> librustdoc/doctree.rs:16:5
[00:47:21] 16 | use rustc_target::spec::abi;
[00:47:21]    |     ^^^^^^^^^^^^^^^^^^^^^^^
[00:47:21] 
[00:47:21] 
[00:47:21] error: unused import: `rustc_target::spec::abi::Abi`
[00:47:21]   --> librustdoc/html/format.rs:22:5
[00:47:21]    |
[00:47:21] 22 | use rustc_target::spec::abi::Abi;
[00:47:21] 
[00:47:21] 
[00:47:21] error: unused import: `rustc_target::spec::abi`
[00:47:21]   --> librustdoc/visit_ast.rs:16:5
[00:47:21] 16 | use rustc_target::spec::abi;
[00:47:21]    |     ^^^^^^^^^^^^^^^^^^^^^^^
[00:47:21] 
7:start=1524753621087041643,finish=1524753621095849794,duration=8808151
