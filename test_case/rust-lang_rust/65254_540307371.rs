abnf
Expr =
  | ...
  | "if" ExprWithLet Block {"else" Block}?
  | {Label ":"}? "while" ExprWithLet Block
  ;

ExprWithLet =
  | "let" PatTop "=" Expr
  | ExprWithLet "&&" ExprWithLet 
  | "(" ExprWithLet ")"
  | Expr
  ;
