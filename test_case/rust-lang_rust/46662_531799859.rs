rust
extern "C" {
    pub type Bar;

    pub fn BAR_is_healthy(bar: *const Bar) -> bool;
}

impl Debug for Bar {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Bar")
            .field("is_healthy", unsafe { &BAR_is_healthy(self) })
            .finish()
    }
}
