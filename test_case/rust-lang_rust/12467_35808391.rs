 rust
/// Bytewise slice equality
#[cfg(not(stage0))]
#[cfg(not(test))]
#[lang("str_eq")]
#[inline]
pub fn eq_slice(a: &str, b: &str) -> bool {
    eq_slice_(a, b)
}
