rust
fn write_fmt<'a>(&'a mut self, fmt: Arguments<'_>) -> WriteFmtFuture<'a, Self>
where
    Self: Unpin;
