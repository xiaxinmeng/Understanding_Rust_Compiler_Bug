rust
   #[rustc_diagnostic_item = "must_use_trait"]
   trait MustUse {
       const REASON: Option<&'static str> = None;
   }
   