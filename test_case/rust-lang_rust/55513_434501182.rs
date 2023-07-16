rust
expr ::= "if" expr "{" statements expr_tail "}"
       | expr "==" expr
       | expr "?"
       | ...
       ;
