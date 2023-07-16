
// Change return type of method indirectly by modifying a type statement------------
mod change_return_type_of_method_indirectly_type {
    #[cfg(cfail1)]
    type ReturnType = super::ReferenceType0;
    #[cfg(not(cfail1))]
    type ReturnType = super::ReferenceType1;

    #[rustc_dirty(label="Hir", cfg="cfail2")]
    #[rustc_clean(label="Hir", cfg="cfail3")]
    #[rustc_metadata_dirty(cfg="cfail2")]
    #[rustc_metadata_clean(cfg="cfail3")]
    trait TraitChangeReturnType {
        fn method() -> ReturnType;
    }
}
