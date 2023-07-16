 rust
let res: Result<u32, ()> = {
    // do some allocations
};
match res {
  Ok(v) => v,
  Err(_) => jmp_outside(),
}
