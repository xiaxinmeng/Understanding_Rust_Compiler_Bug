rust
ParamEnv {
   caller_bounds: [
      Binder(TraitPredicate(<CC2 as std::marker::Sized>, polarity:Positive), []),
      Binder(TraitPredicate(<CC2 as ChainComplex>, polarity:Positive), []),
      Binder(ProjectionPredicate(ProjectionTy { substs: [CC2], item_def_id: DefId(0:7 ~ dep[b688]::ChainComplex::Module) }, FreeModule<usize>), []),
      Binder(TraitPredicate(<CC2 as FreeChainComplex>, polarity:Positive), []),
      Binder(ProjectionPredicate(
         ProjectionTy { substs: [CC2], item_def_id: DefId(0:7 ~ dep[b688]::ChainComplex::Module) },
         FreeModule<<CC2 as ChainComplex>::Algebra>
      ), []),
      Binder(ProjectionPredicate(
         ProjectionTy { substs: [CC2], item_def_id: DefId(0:6 ~ dep[b688]::ChainComplex::Algebra) }, usize), [])
   ],
   reveal: UserFacing
}
