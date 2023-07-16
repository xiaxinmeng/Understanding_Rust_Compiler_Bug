rust
let oks = iter
    .on_error(|e| errs.push(e))
    .collect();
