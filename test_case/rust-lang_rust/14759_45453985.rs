
match self.upgrade() {
    Some(rc) => rc.fmt(f),
    _ => "<invalid reference>".fmt(f)
}
