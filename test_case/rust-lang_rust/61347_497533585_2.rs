java
Item = attrs:OuterAttr* vis:Vis? kind:ItemKind;
ItemKind =
  | ...
  | Const:{ "const" name:IdentOrUnderscore ":" ty:Type "=" value:Expr ";" }
  | ...
  ;

IdentOrUnderscore =
  | Named:IDENT
  | NoName:"_"
  ;
