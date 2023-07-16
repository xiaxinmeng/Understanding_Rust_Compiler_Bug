rust
X[EXPR]
// Becomes
_tmp1 = const &X;
_tmp2 = EXPR;
// bounds check
(*_tmp1)[_tmp2]

match Y { a if GUARD => { /* ... */ }
// Becomes
_tmp3 = const &raw mut Y;
// check guard
a = *_tmp3;
