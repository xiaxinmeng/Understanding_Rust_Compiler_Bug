rust
pub struct RawParser {
    // Unused but necessary; only reproduces when active_tab isn't the
    // first field.
    off: usize,
    active_tab: [u8; 256],
}

impl RawParser {
    pub fn init_active(&mut self) {
        for &c in b"\x00\t\n\r_\\&*[!`<" {
            self.active_tab[c as usize] = 1;
        }
    }
}
