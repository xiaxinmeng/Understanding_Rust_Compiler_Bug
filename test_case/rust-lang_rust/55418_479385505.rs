
#[inline]
#[stable(feature = "spin_loop_hint", since = "1.24.0")]
pub fn spin_loop_hint() {
    spin_loop()
}
