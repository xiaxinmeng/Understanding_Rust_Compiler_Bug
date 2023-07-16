rust
const MUTABLE_BEHIND_RAW: *const Vec<i32> = &Vec::<i32>::new() as *const _;
