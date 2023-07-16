 c++
rust1: internal compiler error: in TyVar, at rust/typecheck/rust-tyty.cc:339
0x78d67f Rust::TyTy::TyVar::TyVar(unsigned int)
        ../gcc/rust/typecheck/rust-tyty.cc:339
0x78d67f Rust::TyTy::TyVar::TyVar(unsigned int)
       ../gcc/rust/typecheck/rust-tyty.cc:333
0xb99a7d Rust::Resolver::TypeCheckType::visit(Rust::HIR::BareFunctionType&)
        ../gcc/rust/typecheck/rust-hir-type-check-type.cc:83
0xb97e01 Rust::Resolver::TypeCheckType::Resolve(Rust::HIR::Type*)
        ../gcc/rust/typecheck/rust-hir-type-check-type.cc:60
0xb9278b Rust::Resolver::TypeCheckItem::visit(Rust::HIR::Function&)
        ../gcc/rust/typecheck/rust-hir-type-check-item.cc:402
0xb8f48c Rust::Resolver::TypeCheckItem::Resolve(Rust::HIR::Item&)
        ../gcc/rust/typecheck/rust-hir-type-check-item.cc:49
0xb8f48c Rust::Resolver::TypeCheckItem::Resolve(Rust::HIR::Item&)
        ../gcc/rust/typecheck/rust-hir-type-check-item.cc:35
0xb5913b Rust::Resolver::TypeResolution::Resolve(Rust::HIR::Crate&)
       ../gcc/rust/typecheck/rust-hir-type-check.cc:37
0xa3b341 Rust::Session::compile_crate(char const*)
        ../gcc/rust/rust-session-manager.cc:604

