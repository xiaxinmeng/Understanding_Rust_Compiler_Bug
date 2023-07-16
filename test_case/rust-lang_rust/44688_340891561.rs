
let x: u8 = ...;
let y: u8 = ...;
// assume &x as usize < &y as usize, to avoid worrying about wrapping in the offset calculation
let ptr_to_y = (&x as *const _).offset(&y as usize - &x as usize);
