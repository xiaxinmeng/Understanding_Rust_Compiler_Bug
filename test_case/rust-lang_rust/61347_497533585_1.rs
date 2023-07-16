java
Item = attrs:OuterAttr* vis:Vis? kind:ItemKind;
ItemKind =
  | ...
  | Const:{ "const" name:IDENT ":" ty:Type "=" value:Expr ";" }
  | ...
  ;
