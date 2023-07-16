rust
#[cfg(not(cfail1))]
#[rustc_clean(label="Hir", cfg="cfail2")]
#[rustc_clean(label="Hir", cfg="cfail3")]
trait TraitChangeModeSelfOwnToMut: Sized {
    #[rustc_dirty(label="Hir", cfg="cfail2")] // changed!
    #[rustc_clean(label="Hir", cfg="cfail3")]
    #[rustc_dirty(label="HirBody", cfg="cfail2")]
    #[rustc_clean(label="HirBody", cfg="cfail3")]
    fn method(mut self) {}
}
