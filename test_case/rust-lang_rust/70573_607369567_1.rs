rust
if index >= len {
  #[cfg(not(bootstrap))]
  panic_bounds_check(index, len);
  #[cfg(bootstrap)]
  panic_bounds_check(Location::caller(), index, len);
}
