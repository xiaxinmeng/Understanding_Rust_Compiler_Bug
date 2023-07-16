rust
impl fmt::Debug for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "stmt({}: {})", self.id.to_string(), pprust::stmt_to_string(self))
    }
}
