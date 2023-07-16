rust
enum StructKind {
   Unit,
   Tuple(Vec<Option<Id>>),
   Normal {
       fields: Vec<Id>,
       fields_stripped: bool,
  },
}
