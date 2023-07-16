rust
fn produce_b(tcx: TyCtxt, def_id: DefId, a: &A) -> (B, C) { }
fn produce_c(tcx: TyCtxt, def_id: DefId, a: A) -> (B, C) { }

// this method will initialize `providers.b` and `providers.c`
// with generated glue functions:
providers.conjoin().take_a().read(produce_b).consume(produce_c);
