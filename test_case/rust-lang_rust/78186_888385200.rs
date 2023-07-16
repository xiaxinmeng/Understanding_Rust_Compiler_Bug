rust
let r = Nasty::new_shared();
// SAFETY: We know for sure *r is never invalidated or moved (or even modified at all).
let p = unsafe { Pin::new_unchecked(r) };
p.check();
