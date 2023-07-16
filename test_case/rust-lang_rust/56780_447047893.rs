
error[E0107]: wrong number of type arguments: expected 1, found 2
  --> diesel_derives/src/util.rs:48:84
   |
48 | pub fn ty_for_foreign_derive(item: &DeriveInput, flags: &MetaItem) -> Result<Type, Diagnostic> {
   |                                                                                    ^^^^^^^^^^ unexpected type argument


