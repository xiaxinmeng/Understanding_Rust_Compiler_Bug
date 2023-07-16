 rust
if /* no padding */ {
    for s in str::from_utf8_lossy(self.as_bytes()) {
        f.buf.write_str(s)
    }
} else {
    let mut collected = str::with_capacity(...);
    // could micro-optimise the case when self is valid utf8 to 
    // (equivalent of) just f.pad(self)
    for s in str::from_utf8_lossy(self.as_bytes()) { collected.push_str(s) }
    f.pad(collected);
}
