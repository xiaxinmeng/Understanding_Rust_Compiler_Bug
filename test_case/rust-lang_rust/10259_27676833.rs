
let mut f: fn(uint) = foo;
if cond { f = bar; }
use(f);
fn use(f: |uint|) { ... }
