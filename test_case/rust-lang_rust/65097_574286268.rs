rust
#[cfg(not(sanitize = "thread"))]
macro_rules! acquire { ($x:expr) => { atomic::fence(Acquire) } }

#[cfg(sanitize = "thread")]
macro_rules! acquire { ($x:expr) => { $x.load(Acquire) } }
