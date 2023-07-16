 rust
let y = x.find('"').unwrap()+1; x.slice(y, y + x.slice_from(y+1).find('"').unwrap()+1)
