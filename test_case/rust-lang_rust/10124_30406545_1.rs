
type AppendTuple<..T, U> = (..T, U);
type PrependTuple<..T, U> = (U, ..T);
AppendTuple<PrependTuple<Tuple<B>, A>, C> => (A, B, C)
