
#[cfg(debug_assertions)]
macro_rules! unreachable_unchecked {
    () => { unreachable!(); };
}
#[cfg(not(debug_assertions))]
macro_rules! unreachable_unchecked {
    () => { ::core::hint::unreachable_unchecked(); };
}
