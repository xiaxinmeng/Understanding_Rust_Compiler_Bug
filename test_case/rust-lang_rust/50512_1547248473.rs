rust
  lhs
= Some(1).xor(Some(2)).xor(Some(3))
= None.xor(Some(3))
= Some(3)

  rhs
= Some(1).xor(Some(2).xor(Some(3)))
= Some(1).xor(None)
= Some(1)
