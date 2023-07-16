 rust
// Non-inlined worker, uses fast in-register calling convention
pub fn skip_whitespace_worker(start: u8*, end: u8*) -> (u8*, u8*, u64) {
  ...
}

// Inlined wrapper, moves reference argument to registers and then calls the worker.
pub fn skip_whitespace(v: &mut std::str::Chars) -> u64 {
  // Approximately:
  let (start, end, len) = skip_whitespace_worker(v.start, v.end);
  mut.start = start;
  mut.end = end;
  len
}
