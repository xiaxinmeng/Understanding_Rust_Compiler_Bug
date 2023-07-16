rust
impl Stdout {
    // These functions do not cause any flushes or i/o interaction of any kind;
    // they simply set a flag that is consulted on each call to `write`. So,
    // transitioning to line_buffered wouldn't try to flush existing unflushed
    // lines until more writes come in (or a manual flush(), obviously).
    fn force_line_buffered(&mut self);
    fn force_block_buffered(&mut self);
}
