
struct Store<'a> {
  a_set: IndexSet<ThingA<'a>>,
  b_set: IndexSet<ThingB<'a>>,
  ..
  n_set: IndexSet<ThingN<'a>>,
}
