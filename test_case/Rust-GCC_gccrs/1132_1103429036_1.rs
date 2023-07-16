
rust1: internal compiler error: in visit_generic_predicates, at rust/privacy/rust-reachability.cc:56
0x74686b Rust::Privacy::ReachabilityVisitor::visit_generic_predicates(std::vector<std::unique_ptr<Rust::HIR::GenericParam, std::default_delete<Rust::HIR::GenericParam> >, std::allocator<std::unique_ptr<Rust::HIR::GenericParam, std::default_delete<Rust::HIR::GenericParam> > > > const&, Rust::Privacy::ReachLevel)
        ../../gcc/rust/privacy/rust-reachability.cc:56
0xb10d68 Rust::Privacy::Resolver::resolve(Rust::HIR::Crate&)
        ../../gcc/rust/privacy/rust-privacy-check.cc:41
0x9ec2ce Rust::Session::parse_file(char const*)
        ../../gcc/rust/rust-session-manager.cc:704
0x9ec94a Rust::Session::parse_files(int, char const**)
        ../../gcc/rust/rust-session-manager.cc:579
Please submit a full bug report,
with preprocessed source if appropriate.
Please include the complete backtrace with any bug report.
See <https://gcc.gnu.org/bugs/> for instructions.
