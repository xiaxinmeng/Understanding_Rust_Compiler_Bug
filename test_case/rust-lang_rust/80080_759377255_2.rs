rust
type Alias = <WithStructStruct as Trait>::AssociatedType;
let alias = Alias { a: 0 } ;
match alias {
  Alias { a } => a,
};
