rust
const NASTY: (bool, *const i32, *const i32) {
  let x = 0; let y = 1;
  let x = &x as *const _; let y = &y as *const _;
  (x == y, x, y)
}
