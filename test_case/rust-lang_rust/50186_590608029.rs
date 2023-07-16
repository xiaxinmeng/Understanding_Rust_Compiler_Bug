rust
fn update<F>(&self, f: F) where T: Copy, F: FnOnce(T) -> T;
fn update_map<F, U>(&self, f: F) -> U where T: Copy, F: FnOnce(T) -> (T, U);
fn take_update<F>(&self, f: F) where T: Default, F: FnOnce(T) -> T;
fn take_update_map<F, U>(&self, f: F) -> U where T: Default, F: FnOnce(T) -> (T, U);
