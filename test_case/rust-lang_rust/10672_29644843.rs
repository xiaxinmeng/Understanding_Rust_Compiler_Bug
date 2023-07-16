
fn shim(self: *Self, ...) {
    unsafe { actual(*self, ...) }
}
