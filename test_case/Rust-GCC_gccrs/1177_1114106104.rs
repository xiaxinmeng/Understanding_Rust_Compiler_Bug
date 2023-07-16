
Crate: 
 inner attributes: none
 items: 
  
i32 main()
BlockExpr:

  outer attributes: none
  inner attributes: none
 statements: 
 
  outer attributes: none
 let mut res = 0
 E
 Generic params: none
 Where clause: none
 Items: 
  X(
  outer attributes: none
 u8)

  outer attributes: none
 let v = CallExpr: 
  outer attributes: none
 Function expr: E::X
 Call params:
  4
  ExprStmtWithBlock: 
IfLetExpr: 
    outer attributes: none
 Condition match arm patterns: 
  TupleStructPattern: 
 Path: E::X
 Tuple struct items: 
  n
 Scrutinee expr: v
 If let block expr:    BlockExpr:
   
     outer attributes: none
     inner attributes: none
    statements: 
    ExprStmtWithoutBlock:
     AssignmentExpr: 
 left: res
 right: n
    final expression: none
   
 final expression: 
0
