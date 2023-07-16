 rust
impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RGB ({red}, {green}, {blue}) 0x{red:02X}{green:02X}{blue:02X}", red = self.red, green = self.green, blue = self.blue)
    }
}
