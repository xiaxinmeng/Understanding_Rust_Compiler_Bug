 rust
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.flags & (1 << FlagAlternate) {
            write!(f.buf, "({:#}, {:#})", self.x, self.y)
        } else { 
            write!(f.buf, "({}, {})", self.x, self.y)
        }
    }
