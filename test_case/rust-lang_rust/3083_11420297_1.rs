 rust
umc(var, if cond { blk1 } else { blk2 }) = max(umc(var, blk1), umc(var, blk2))
umc(var, { stmt1; stmt2; ... }) = umc(var, stmt1) + umc(var, stmt2) + ...
umc(var, while expr { body }) = if umc(var, body) > Known(0) || umc(var, expr) > Known(0) { Unknown } else { Known(0) }
umc(var, let var = ...;) = Known(1)
umc(var, let var;) = Known(0)
