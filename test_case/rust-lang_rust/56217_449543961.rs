
Float  ::= Sign? ( 'inf' | 'NaN' | Number )
Number ::= ( Digit+ |
             Digit+ '.' Digit* |
             Digit* '.' Digit+ ) Exp?
Exp    ::= [eE] Sign? Digit+
Sign   ::= [+-]
Digit  ::= [0-9]
