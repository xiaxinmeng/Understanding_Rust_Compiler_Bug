
#[cfg(not(debug_assertions))]
macro_rules! unreachable {
    () => { unsafe { ::core::hint::unreachable_unchecked() } };
}
