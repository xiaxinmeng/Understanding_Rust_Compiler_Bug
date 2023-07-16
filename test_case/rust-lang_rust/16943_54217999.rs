 rust
impl fmt::Show for S {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::FormatError> {
        formatter.write_str("0o")
            .map_err(|_|fmt::WriteError)
            .and_then(|_|(&self.bits as &fmt::Octal).fmt(formatter))
    }
}
