 rust
let y = x.find('"').unwrap()+1; x.slice(y, x.find('"', y).unwrap())
