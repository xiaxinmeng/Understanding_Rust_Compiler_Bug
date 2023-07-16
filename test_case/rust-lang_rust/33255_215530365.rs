
BinaryOp(BinOp, Operand, Operand, bool) -trans-to-> (ValueRef, ValueRef)
                                  ^^^^               ^^^^^^^^  ^^^^^^^^ The overflow flag (plain undef in case bool = false)
                                   |                 └ The result value
                                   Whether the operation should use overflow checks
