rust
if def_id.is_local() && tcx.typeck_tables_of(def_id).tainted_by_errors {
    return Err(ErrorHandled::Reported);
}
