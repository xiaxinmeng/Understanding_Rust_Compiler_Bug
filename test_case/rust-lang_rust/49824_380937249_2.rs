rust
struct ClosureStruct<'x, 'y, 'a, 'b> {
  x: &'x mut Vec<&'a u32>,
  y: &'y &'b u32
}

impl FnOnce for ClosureStruct { .. }
