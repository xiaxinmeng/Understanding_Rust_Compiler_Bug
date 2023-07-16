
If T appears in the impl trait reference,
  then: T is constrained

If T appears in the impl self type,
  then: T is constrained

If <T0 as Trait<T1...Tn>>::U == V appears in the impl predicates,
  and T0...Tn are constrained
  and T0 as Trait<T1...Tn> is not the impl trait reference
  then: V is constrained
