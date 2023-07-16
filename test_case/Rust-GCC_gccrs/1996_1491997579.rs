
$ crab1 -frust-incomplete-and-experimental-compiler-do-not-use /tmp/break-rust.rs
crab1: internal compiler error: at break-rust.rs:2:11, oopsie woopsie! you broke GCC Rust
0x9c938c rust_be_internal_error_at(Location, std::__cxx11::basic_string<char, std::char_traits<char>, std::allocator<char> > const&)
        ../../gcc/rust/rust-diagnostics.cc:166
0x9c9430 rust_internal_error_at(Location, char const*, ...)
        ../../gcc/rust/rust-diagnostics.cc:175
0xb9c366 Rust::Resolver::ResolveExpr::visit(Rust::AST::IdentifierExpr&)
        ../../gcc/rust/resolve/rust-ast-resolve-expr.cc:126
0xb9ce59 Rust::Resolver::ResolveExpr::go(Rust::AST::Expr*, Rust::Resolver::CanonicalPath const&, Rust::Resolver::CanonicalPath const&, bool)
        ../../gcc/rust/resolve/rust-ast-resolve-expr.cc:36
0xb9ce59 Rust::Resolver::ResolveExpr::visit(Rust::AST::BreakExpr&)
        ../../gcc/rust/resolve/rust-ast-resolve-expr.cc:402
0xb9d32b Rust::Resolver::ResolveExpr::go(Rust::AST::Expr*, Rust::Resolver::CanonicalPath const&, Rust::Resolver::CanonicalPath const&, bool)
        ../../gcc/rust/resolve/rust-ast-resolve-expr.cc:36
0xb9ba88 Rust::Resolver::ResolveStmt::go(Rust::AST::Stmt*, Rust::Resolver::CanonicalPath const&, Rust::Resolver::CanonicalPath const&, Rust::Resolver::CanonicalPath const&)
        ../../gcc/rust/resolve/rust-ast-resolve-stmt.h:43
0xb9ba88 Rust::Resolver::ResolveStmt::go(Rust::AST::Stmt*, Rust::Resolver::CanonicalPath const&, Rust::Resolver::CanonicalPath const&, Rust::Resolver::CanonicalPath const&)
        ../../gcc/rust/resolve/rust-ast-resolve-stmt.h:35
0xb9ba88 Rust::Resolver::ResolveExpr::visit(Rust::AST::BlockExpr&)
        ../../gcc/rust/resolve/rust-ast-resolve-expr.cc:254
0xb9d32b Rust::Resolver::ResolveExpr::go(Rust::AST::Expr*, Rust::Resolver::CanonicalPath const&, Rust::Resolver::CanonicalPath const&, bool)
        ../../gcc/rust/resolve/rust-ast-resolve-expr.cc:36
0xb90055 Rust::Resolver::ResolveItem::visit(Rust::AST::Function&)
        ../../gcc/rust/resolve/rust-ast-resolve-item.cc:532
0xb8c6c3 Rust::Resolver::ResolveItem::go(Rust::AST::Item*, Rust::Resolver::CanonicalPath const&, Rust::Resolver::CanonicalPath const&)
        ../../gcc/rust/resolve/rust-ast-resolve-item.cc:210
0xb82264 Rust::Resolver::NameResolution::go(Rust::AST::Crate&)
        ../../gcc/rust/resolve/rust-ast-resolve.cc:108
0xa927f1 Rust::Session::compile_crate(char const*)
        ../../gcc/rust/rust-session-manager.cc:587
Please submit a full bug report, with preprocessed source (by using -freport-bug).
Please include the complete backtrace with any bug report.
See <https://gcc.gnu.org/bugs/> for instructions.
