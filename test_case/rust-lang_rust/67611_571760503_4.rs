rust
X[EXPR]
// Becomes
_tmp2 = EXPR;
// bounds check
_tmp1 = const &X;
(*_tmp1)[_tmp2]
