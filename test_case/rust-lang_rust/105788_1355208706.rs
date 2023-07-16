console
[
    (Binder(TraitPredicate(<Self as Foo>, polarity:Positive), []), Some(Binder(TraitPredicate(<&Self as Foo>, polarity:Positive), [])), Some(ObligationCause { span: ./p/e.rs:9:14: 9:15 (#0), body_id: HirId { owner: OwnerId { def_id: DefId(0:6 ~ e[8a4c]::Bar::f) }, local_id: 9 }, code: BuiltinDerivedObligation(DerivedObligationCause { parent_trait_pred: Binder(TraitPredicate(<_ as Foo>, polarity:Positive), []), parent_code: MiscObligation }) })),

    (Binder(TraitPredicate(<&Self as Foo>, polarity:Positive), []), None, Some(ObligationCause { span: ./p/e.rs:9:14: 9:15 (#0), body_id: HirId { owner: OwnerId { def_id: DefId(0:6 ~ e[8a4c]::Bar::f) }, local_id: 9 }, code: ImplDerivedObligation(ImplDerivedObligationCause { derived: DerivedObligationCause { parent_trait_pred: Binder(TraitPredicate(<_ as Foo>, polarity:Positive), []), parent_code: BuiltinDerivedObligation(DerivedObligationCause { parent_trait_pred: Binder(TraitPredicate(<_ as Foo>, polarity:Positive), []), parent_code: MiscObligation }) }, impl_def_id: DefId(0:3 ~ e[8a4c]::Foo), span: no-location (#0) }) }))
]
