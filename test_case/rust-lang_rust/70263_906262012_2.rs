rust
use parking_lot::{Mutex, MutexGuard};

let m = Mutex::new((0, 0));
let guard = MutexGuard::map(
    m.lock(),
    |inner: &mut (usize, usize)| -> &mut usize { &mut inner.0 },
);
