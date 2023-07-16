
error[E0423]: expected value, found struct `X`
  |
4 |     if let X {} = X {} {}
  |                   ^
help: if you wanted to use a literal struct constructor here, try surrounding it with parenthesis
4 |     if let X {} = (X {}) {}
  |                   ^^^^^^
