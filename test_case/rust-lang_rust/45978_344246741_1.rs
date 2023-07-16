rust
if let None = file { return; }
let Some(f) = file;
// Acceptable, we know that `Option` can only be 
// `None` or `Some(_)`, and the `None` case will not reach here
// so we are sure that `file` can only be `Some(_)`
// and thus the pattern is irrefutable. 
