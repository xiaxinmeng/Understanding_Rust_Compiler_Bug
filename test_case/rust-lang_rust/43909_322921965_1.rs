rust
impl Debug for str {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.write_char('"')?;
        let mut from = 0;
        for (i, c) in self.char_indices() {
            let esc = c.escape_debug();
            // If char needs escaping, flush backlog so far and write, else skip
            if esc.len() != 1 {
                f.write_str(&self[from..i])?;
                for c in esc {
                    f.write_char(c)?;
                }
                from = i + c.len_utf8();
            }
        }
        f.write_str(&self[from..])?;
        f.write_char('"')
    }
}
