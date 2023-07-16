rust
let id = next_id.update(|x| { let y = *x; *x += 1; y });
