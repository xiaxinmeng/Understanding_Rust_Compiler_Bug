
beginning null denotation identifier handling
current peek token when starting path pratt parse: '|='
current token (just about to return path to null denotation): '|='
finished null denotation identifier path parsing - next is branching 
test.rs:24:7: error: expecting ‘}’ but ‘|=’ found
   24 |     g |= 7;
      |       ^
test.rs:24:5: error: error may be from having an expression (as opposed to statement) in the body of the function but not last
   24 |     g |= 7;
      |     ^

