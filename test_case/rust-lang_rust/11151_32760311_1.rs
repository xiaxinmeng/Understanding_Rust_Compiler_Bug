
  #[feature(managed_boxes, globs, macro_registrar, macro_rules)];

  pub mod inner;

  #[macro_export]
  macro_rules! exported_macro (() => (1))
