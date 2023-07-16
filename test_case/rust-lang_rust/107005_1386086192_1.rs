rust
let n = 32_usize;
const LIMIT: usize = u8::MAX as usize;
let bug = match n {
    n @ ..= LIMIT => true,
    _ => false,
};

