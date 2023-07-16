rust
Crate: 
 inner attributes: none
 items: 
  
u32 abc(x : u32, y : u32)
BlockExpr:
{
 outer attributes: none
 inner attributes: none
 statements: 
 ExprStmtWithoutBlock: return ArithmeticOrLogicalExpr: x + y
 final expression: none
}

  
void main()
BlockExpr:
{
 outer attributes: none
 inner attributes: none
 statements: 
  ExprStmtWithBlock: 
   BlockExpr:
   {
    outer attributes: none
    inner attributes: none
    statements: 
    ExprStmtWithoutBlock: ArithmeticOrLogicalExpr: 1 + 1
    final expression: none
   }
 ExprStmtWithoutBlock: println!("Hello World!")
 final expression: none
}
