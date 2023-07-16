rust
OuterAttr = "#" attr:Attr;
InnerAttr = "#" "!" attr:Attr;
Attr = "[" path:Path input:AttrInput? "]";
AttrInput =
  | "(" TOKEN_STREAM ")"
  | "[" TOKEN_STREAM "]"
  | "{" TOKEN_STREAM "}"
  | "=" LITERAL // Unsuffixed literals only.
  ;
