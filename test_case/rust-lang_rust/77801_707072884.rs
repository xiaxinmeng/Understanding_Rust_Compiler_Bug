rust
fn get_or_init_in_place(&'static self, f: impl FnOnce() -> T, g: impl FnOnce(&'static T)) -> &'static T
