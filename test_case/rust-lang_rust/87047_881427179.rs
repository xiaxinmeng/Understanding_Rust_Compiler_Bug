rust
let oks = iter
    .flat_map(|r| r.map_err(|e| errs.push(e)))
    .collect();
