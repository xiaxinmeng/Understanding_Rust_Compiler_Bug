
LVALUE_EXPR = 
       MUTABILITY RVALUE_EXPR           - temporary
       | LVALUE_EXPR . FIELD            - field access
       | LVALUE_EXPR [ RVALUE_EXPR ]    - indexing (including overloaded?)
       | *MUTABILITY LVALUE_EXPR        - deref
       | LOCAL                          - local variable
