rust
fn legal(x: &mut i32) { unsafe {
  let x = x as *mut _ as usize;
  let y1 = &mut *(x as *mut i32);
  let y2 = &mut *(y1 as *mut i32 as usize as *mut i32);
  // these two writes are legal -- y2 is reborrowing from y1, but because 
  // we went though usize we cannot possibly track where they are "derived from"
  *y2 = 4;
  *y1 = 3;
  // Using y2 again here would be UB
} }

fn illegal(x: &mut i32) { unsafe {
  let x = x as *mut _ as usize;
  let y1 = &mut *(x as *mut i32);
  let y2 = &mut *(y1 as *mut i32 as usize as *mut i32);

  bar(y1, y2); // this call must trigger UB
} }
fn bar(y1: &mut i32, y2: &mut i32) {
  // we will certainly want to optimize this
  *y2 = 4;
  *y1 = 3;
}
