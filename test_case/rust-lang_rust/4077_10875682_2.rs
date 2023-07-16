
fn repeat(i: uint, action: fn()) {
     if i > 0 {
         action();
         repeat(i - 1, action);
     }
}
