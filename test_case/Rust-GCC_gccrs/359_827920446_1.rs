
gccrs/gcc/testsuite/rust.test/unsupported/lifetime_param2.rs:2:13: error: failed to setup generic parameter
rust1: internal compiler error: in insert_type, at rust/typecheck/rust-tyctx.cc:79
0x10cffbc Rust::Resolver::TypeCheckContext::insert_type(Rust::Analysis::NodeMapping const&, Rust::TyTy::BaseType*)
        ../../gcc/rust/typecheck/rust-tyctx.cc:79
0xffe1cd Rust::HIR::StructStruct::accept_vis(Rust::HIR::HIRVisitor&)
        ../../gcc/rust/hir/tree/rust-hir-full-test.cc:5531
0x109af62 Rust::Resolver::TypeResolution::Resolve(Rust::HIR::Crate&)
        ../../gcc/rust/typecheck/rust-hir-type-check.cc:38
0xf1290f Rust::Session::parse_file(char const*)
        ../../gcc/rust/rust-session-manager.cc:561
0xf12500 Rust::Session::parse_files(int, char const**)
        ../../gcc/rust/rust-session-manager.cc:451
0xe7e0b4 grs_langhook_parse_file
        ../../gcc/rust/rust-lang.cc:170
Please submit a full bug report,
with preprocessed source if appropriate.
Please include the complete backtrace with any bug report.
See <https://gcc.gnu.org/bugs/> for instructions.
