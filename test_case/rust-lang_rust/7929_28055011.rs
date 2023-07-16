
   struct Ast {
       items: ~[Item],
       exprs: ~[Expr],
       ...
   }
   
   ...
   
   enum Expr {
        ...
        ExprBinop(Binop, ExprId, ExprId),
        ...
   }
   
   struct ItemId { index: u32 }
   struct ExprId { index: u32 }
   