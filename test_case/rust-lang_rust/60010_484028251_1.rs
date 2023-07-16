
DEBUG 2019-04-17T09:14:41Z: rustc::traits::select: evaluate_stack entry
stack: TraitObligationStack { obligation: Obligation(predicate=Binder(TraitPredicate(<RootDatabase as SourceDatabase>)),depth=9), fresh_trait_ref: Binder(<RootDatabase as SourceDatabase>) }
    :: TraitObligationStack { obligation: Obligation(predicate=Binder(TraitPredicate(<SalsaStorage as std::panic::RefUnwindSafe>)),depth=7), fresh_trait_ref: Binder(<SalsaStorage as std::panic::RefUnwindSafe>) }
    :: TraitObligationStack { obligation: Obligation(predicate=Binder(TraitPredicate(<*const SalsaStorage as std::panic::RefUnwindSafe>)),depth=6), fresh_trait_ref: Binder(<*const SalsaStorage as std::panic::RefUnwindSafe>) }
    :: TraitObligationStack { obligation: Obligation(predicate=Binder(TraitPredicate(<core::nonzero::NonZero<*const SalsaStorage> as std::panic::RefUnwindSafe>)),depth=5), fresh_trait_ref: Binder(<core::nonzero::NonZero<*const SalsaStorage> as std::panic::RefUnwindSafe>) }
    :: TraitObligationStack { obligation: Obligation(predicate=Binder(TraitPredicate(<std::ptr::Unique<SalsaStorage> as std::panic::RefUnwindSafe>)),depth=4), fresh_trait_ref: Binder(<std::ptr::Unique<SalsaStorage> as std::panic::RefUnwindSafe>) }
    :: TraitObligationStack { obligation: Obligation(predicate=Binder(TraitPredicate(<std::boxed::Box<SalsaStorage> as std::panic::RefUnwindSafe>)),depth=3), fresh_trait_ref: Binder(<std::boxed::Box<SalsaStorage> as std::panic::RefUnwindSafe>) }
    :: TraitObligationStack { obligation: Obligation(predicate=Binder(TraitPredicate(<Runtime<RootDatabase> as std::panic::RefUnwindSafe>)),depth=2), fresh_trait_ref: Binder(<Runtime<RootDatabase> as std::panic::RefUnwindSafe>) }
    :: TraitObligationStack { obligation: Obligation(predicate=Binder(TraitPredicate(<RootDatabase as std::panic::RefUnwindSafe>)),depth=1), fresh_trait_ref: Binder(<RootDatabase as std::panic::RefUnwindSafe>) }
    :: TraitObligationStack { obligation: Obligation(predicate=Binder(TraitPredicate(<RootDatabase as SourceDatabase>)),depth=0), fresh_trait_ref: Binder(<RootDatabase as SourceDatabase>) }
    :: []
DEBUG 2019-04-17T09:14:41Z: rustc::traits::select: evaluate_stack(Binder(<RootDatabase as SourceDatabase>)) --> recursive
