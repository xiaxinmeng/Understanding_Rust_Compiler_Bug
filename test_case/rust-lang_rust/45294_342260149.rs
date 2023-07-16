
T = T + U
U = Path
  | &U
  | impl U
  | dyn U
  | Box<T>
  | (T)
  | fn(T...) -> U 

// bounds
B = C | B + C
C = Path "<" T... ">"
  | Path "(" T... ")" -> U
