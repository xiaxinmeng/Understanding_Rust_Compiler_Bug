bash
175  | /  macro_rules! define_callbacks {
176  | |      (
177  | |       $($(#[$attr:meta])*
178  | |          [$($modifiers:tt)*] fn $name:ident($($K:tt)*) -> $V:ty,)*) => {
...    |
282  | |                          if key.query_crate_is_local() { "local" } else { "external" } ,
     | |                                 ^^^^^^^^^^^^^^^^^^^^ method not found in `(ty::Ty<'_>, Option<Binder<'_, ExistentialTraitRef<'_>>>)`
...    |
321  | |      };
322  | |  }
     | |__- in this expansion of `define_callbacks!` (#2)
