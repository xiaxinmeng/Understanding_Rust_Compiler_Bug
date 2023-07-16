
#[cfg("cfail1")]
pub struct EmptyStruct;

#[rustc_clean(label="Hir", cfg="cfail2")]
#[rustc_metadata_clean(cfg="cfail2")]
pub struct EmptyStruct;
