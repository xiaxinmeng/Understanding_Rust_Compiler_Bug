
In file included from ../../gcc/rust/typecheck/rust-hir-type-check-type.h:25:0,
                 from ../../gcc/rust/typecheck/rust-hir-trait-resolve.h:25,
                 from ../../gcc/rust/backend/rust-compile.cc:19:
../../gcc/rust/typecheck/rust-hir-path-probe.h: In member function ‘void Rust::Resolver::PathProbeType::process_associated_trait_for_candidates(const Rust::Resolver::TraitReference*, Rust::HIR::ImplBlock*, bool)’:
../../gcc/rust/typecheck/rust-hir-path-probe.h:370:28: error: no matching function for call to ‘Rust::Resolver::PathProbeCandidate::PathProbeCandidate(<brace-enclosed initializer list>)’
      {trait_item_candidate}};
                            ^
../../gcc/rust/typecheck/rust-hir-path-probe.h:370:28: note: candidates are:
../../gcc/rust/typecheck/rust-hir-path-probe.h:91:3: note: Rust::Resolver::PathProbeCandidate::PathProbeCandidate(Rust::Resolver::PathProbeCandidate::CandidateType, Rust::TyTy::BaseType*, Location, Rust::Resolver::PathProbeCandidate::TraitItemCandidate)
   PathProbeCandidate (CandidateType type, TyTy::BaseType *ty, Location locus,
   ^
../../gcc/rust/typecheck/rust-hir-path-probe.h:91:3: note:   no known conversion for argument 4 from ‘<brace-enclosed initializer list>’ to ‘Rust::Resolver::PathProbeCandidate::TraitItemCandidate’
../../gcc/rust/typecheck/rust-hir-path-probe.h:86:3: note: Rust::Resolver::PathProbeCandidate::PathProbeCandidate(Rust::Resolver::PathProbeCandidate::CandidateType, Rust::TyTy::BaseType*, Location, Rust::Resolver::PathProbeCandidate::ImplItemCandidate)
   PathProbeCandidate (CandidateType type, TyTy::BaseType *ty, Location locus,
   ^
../../gcc/rust/typecheck/rust-hir-path-probe.h:86:3: note:   no known conversion for argument 4 from ‘<brace-enclosed initializer list>’ to ‘Rust::Resolver::PathProbeCandidate::ImplItemCandidate’
../../gcc/rust/typecheck/rust-hir-path-probe.h:81:3: note: Rust::Resolver::PathProbeCandidate::PathProbeCandidate(Rust::Resolver::PathProbeCandidate::CandidateType, Rust::TyTy::BaseType*, Location, Rust::Resolver::PathProbeCandidate::EnumItemCandidate)
   PathProbeCandidate (CandidateType type, TyTy::BaseType *ty, Location locus,
   ^
../../gcc/rust/typecheck/rust-hir-path-probe.h:81:3: note:   no known conversion for argument 4 from ‘<brace-enclosed initializer list>’ to ‘Rust::Resolver::PathProbeCandidate::EnumItemCandidate’
../../gcc/rust/typecheck/rust-hir-path-probe.h:31:8: note: constexpr Rust::Resolver::PathProbeCandidate::PathProbeCandidate(const Rust::Resolver::PathProbeCandidate&)
 struct PathProbeCandidate
        ^
../../gcc/rust/typecheck/rust-hir-path-probe.h:31:8: note:   candidate expects 1 argument, 4 provided
../../gcc/rust/typecheck/rust-hir-path-probe.h:31:8: note: constexpr Rust::Resolver::PathProbeCandidate::PathProbeCandidate(Rust::Resolver::PathProbeCandidate&&)
../../gcc/rust/typecheck/rust-hir-path-probe.h:31:8: note:   candidate expects 1 argument, 4 provided
../../gcc/rust/typecheck/rust-hir-path-probe.h: In member function ‘void Rust::Resolver::PathProbeType::process_predicate_for_candidates(const Rust::TyTy::TypeBoundPredicate&, bool)’:
../../gcc/rust/typecheck/rust-hir-path-probe.h:415:28: error: no matching function for call to ‘Rust::Resolver::PathProbeCandidate::PathProbeCandidate(<brace-enclosed initializer list>)’
      {trait_item_candidate}};
                            ^
../../gcc/rust/typecheck/rust-hir-path-probe.h:415:28: note: candidates are:
../../gcc/rust/typecheck/rust-hir-path-probe.h:91:3: note: Rust::Resolver::PathProbeCandidate::PathProbeCandidate(Rust::Resolver::PathProbeCandidate::CandidateType, Rust::TyTy::BaseType*, Location, Rust::Resolver::PathProbeCandidate::TraitItemCandidate)
   PathProbeCandidate (CandidateType type, TyTy::BaseType *ty, Location locus,
   ^
../../gcc/rust/typecheck/rust-hir-path-probe.h:91:3: note:   no known conversion for argument 4 from ‘<brace-enclosed initializer list>’ to ‘Rust::Resolver::PathProbeCandidate::TraitItemCandidate’
../../gcc/rust/typecheck/rust-hir-path-probe.h:86:3: note: Rust::Resolver::PathProbeCandidate::PathProbeCandidate(Rust::Resolver::PathProbeCandidate::CandidateType, Rust::TyTy::BaseType*, Location, Rust::Resolver::PathProbeCandidate::ImplItemCandidate)
   PathProbeCandidate (CandidateType type, TyTy::BaseType *ty, Location locus,
   ^
../../gcc/rust/typecheck/rust-hir-path-probe.h:86:3: note:   no known conversion for argument 4 from ‘<brace-enclosed initializer list>’ to ‘Rust::Resolver::PathProbeCandidate::ImplItemCandidate’
../../gcc/rust/typecheck/rust-hir-path-probe.h:81:3: note: Rust::Resolver::PathProbeCandidate::PathProbeCandidate(Rust::Resolver::PathProbeCandidate::CandidateType, Rust::TyTy::BaseType*, Location, Rust::Resolver::PathProbeCandidate::EnumItemCandidate)
   PathProbeCandidate (CandidateType type, TyTy::BaseType *ty, Location locus,
   ^
../../gcc/rust/typecheck/rust-hir-path-probe.h:81:3: note:   no known conversion for argument 4 from ‘<brace-enclosed initializer list>’ to ‘Rust::Resolver::PathProbeCandidate::EnumItemCandidate’
../../gcc/rust/typecheck/rust-hir-path-probe.h:31:8: note: constexpr Rust::Resolver::PathProbeCandidate::PathProbeCandidate(const Rust::Resolver::PathProbeCandidate&)
 struct PathProbeCandidate
        ^
../../gcc/rust/typecheck/rust-hir-path-probe.h:31:8: note:   candidate expects 1 argument, 4 provided
../../gcc/rust/typecheck/rust-hir-path-probe.h:31:8: note: constexpr Rust::Resolver::PathProbeCandidate::PathProbeCandidate(Rust::Resolver::PathProbeCandidate&&)
../../gcc/rust/typecheck/rust-hir-path-probe.h:31:8: note:   candidate expects 1 argument, 4 provided
