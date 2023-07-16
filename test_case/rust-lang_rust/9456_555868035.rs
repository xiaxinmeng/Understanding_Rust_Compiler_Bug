rust
impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "RGB ({0}, {1}, {2}) 0x{0:02X}{1:02X}{2:02X}",
            self.red, self.green, self.blue
        )
    }
}
