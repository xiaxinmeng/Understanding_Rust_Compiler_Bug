
fn id(x: int) -> int { x }
fn main() { assert (id as native fn(int)->int)(3) == 3; }
