
tcx.queries.def_span.iter_results(|iter| {
  for (k, _, v) in iter {
    if !k.is_local() { continue }
    record!(self.tables.def_span[k] <- v);
  }
}
