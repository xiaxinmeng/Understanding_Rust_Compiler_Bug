
fun_body : LBRACE inner_attr* block_element* block_last_element? RBRACE ;
block_element : expr_no_lhs_stmt (SEMI)+
  | stmt_not_just_expr (SEMI)*
  ;

// a statement that is not parsed by the expr_no_lhs_stmt rule
stmt_not_just_expr : let_stmt
  | macro_expr
  | mod_item
  | expr_stmt
  ;

block_last_element : expr_no_lhs_stmt | mac_expr | expr_stmt ;
