c
#0  rust_error_at (location=..., fmt=fmt@entry=0x22d9800 "failed to find name: %s") at ../../gcc/rust/rust-diagnostics.cc:188
#1  0x0000000000b82afb in Rust::Resolver::ResolveExpr::visit (this=0x7fffffffd580, expr=...) at ../../gcc/rust/resolve/rust-ast-resolve-expr.cc:125
#2  0x0000000000b83667 in Rust::Resolver::ResolveExpr::go (canonical_prefix=..., prefix=..., expr=0x32043f0) at ../../gcc/rust/resolve/rust-ast-resolve-expr.cc:35
#3  Rust::Resolver::ResolveExpr::visit (this=<optimized out>, expr=...) at ../../gcc/rust/resolve/rust-ast-resolve-expr.cc:387
#4  0x0000000000b83a94 in Rust::Resolver::ResolveExpr::go (expr=0x3204370, prefix=..., canonical_prefix=...) at ../../gcc/rust/resolve/rust-ast-resolve-expr.cc:35
#5  0x0000000000b82339 in Rust::Resolver::ResolveStmt::go (enum_prefix=..., canonical_prefix=..., prefix=..., stmt=0x3204350) at ../../gcc/rust/resolve/rust-ast-resolve-stmt.h:43
#6  Rust::Resolver::ResolveStmt::go (enum_prefix=..., canonical_prefix=..., prefix=..., stmt=0x3204350) at ../../gcc/rust/resolve/rust-ast-resolve-stmt.h:35
#7  Rust::Resolver::ResolveExpr::visit (this=0x7fffffffd700, expr=...) at ../../gcc/rust/resolve/rust-ast-resolve-expr.cc:248
#8  0x0000000000b83a94 in Rust::Resolver::ResolveExpr::go (expr=0x31d8c10, prefix=..., canonical_prefix=...) at ../../gcc/rust/resolve/rust-ast-resolve-expr.cc:35
#9  0x0000000000b76a04 in Rust::Resolver::ResolveItem::visit (this=0x7fffffffd890, function=...) at ../../gcc/rust/ast/rust-item.h:1655
#10 0x0000000000b72fe4 in Rust::Resolver::ResolveItem::go (item=0x31dc0a0, prefix=..., canonical_prefix=...) at ../../gcc/rust/resolve/rust-ast-resolve-item.cc:200
#11 0x0000000000b68bb5 in Rust::Resolver::NameResolution::go (this=0x3207840, crate=...) at /usr/include/c++/10/bits/unique_ptr.h:173
#12 0x0000000000a77032 in Rust::Session::compile_crate (this=0x2fdc460 <Rust::Session::get_instance()::instance>, filename=<optimized out>) at ../../gcc/rust/rust-session-manager.cc:577
#13 0x000000000117c84e in compile_file () at ../../gcc/toplev.cc:444
#14 0x00000000009bad04 in do_compile (no_backend=false) at ../../gcc/toplev.cc:2125
#15 toplev::main (this=this@entry=0x7fffffffe0de, argc=<optimized out>, argc@entry=22, argv=<optimized out>, argv@entry=0x7fffffffe1e8) at ../../gcc/toplev.cc:2277
#16 0x00000000009bd1af in main (argc=22, argv=0x7fffffffe1e8) at ../../gcc/main.cc:39
