
S = X[0]
C = 0
for i in [1..N]:
  Y = X[i] - C
  T = S + Y
  C = (T - S) - Y
  S = T
