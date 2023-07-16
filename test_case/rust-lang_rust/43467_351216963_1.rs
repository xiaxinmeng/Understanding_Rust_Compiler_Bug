rust
pub extern type CFString;
fn do_stuff_with_shared_string(str: &CFShared<CFString>) { ... }
