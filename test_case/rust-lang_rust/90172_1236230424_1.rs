rust
self
    .map_err(E::from)
    .map_err(Report::new)
    .map_err(|report| report.pretty(true).show_backtrace(true))
            