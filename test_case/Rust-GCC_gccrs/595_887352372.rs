
$ bin/gccrs ../src/gcc/testsuite/rust/compile/torture/unsafe1.rs 
Preparing to parse files. 
Attempting to parse file: ../src/gcc/testsuite/rust/compile/torture/unsafe1.rs
beginning null denotation identifier handling
current peek token when starting path pratt parse: '='
current token (just about to return path to null denotation): '='
finished null denotation identifier path parsing - next is branching 
beginning null denotation identifier handling
current peek token when starting path pratt parse: '}'
current token (just about to return path to null denotation): '}'
finished null denotation identifier path parsing - next is branching 
beginning null denotation identifier handling
current peek token when starting path pratt parse: '='
current token (just about to return path to null denotation): '='
finished null denotation identifier path parsing - next is branching 
beginning null denotation identifier handling
current peek token when starting path pratt parse: '('
current token (just about to return path to null denotation): '('
finished null denotation identifier path parsing - next is branching 
SUCCESSFULLY PARSED CRATE 
ran register_plugins (with no body)
SUCCESSFULLY REGISTERED PLUGINS 
started injection
finished injection
SUCCESSFULLY FINISHED INJECTION 
started expansion
finished expansion
SUCCESSFULLY FINISHED EXPANSION 
../src/gcc/testsuite/rust/compile/torture/unsafe1.rs:2:5: fatal error: Failed to lower expr: [UnsafeBlockExpr:
  outer attributes: noneBlockExpr:

  outer attributes: none
  inner attributes: none
 statements: 
 
  outer attributes: none
 let a
 ExprStmtWithoutBlock:
  AssignmentExpr: 
 left: a
 right: 123
 final expression: 
a

]
    2 |     unsafe {
      |     ^
compilation terminated.
