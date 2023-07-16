rust
use async_std::future;

let a = future::pending();
let b = future::ready(1u8);

assert_eq!(a.race(b).await, 1u8);
