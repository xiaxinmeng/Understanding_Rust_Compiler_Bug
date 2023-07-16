rust
unsafe impl JSTraceable for Filter {
    unsafe fn trace(&self, tracer: *mut JSTracer) {
        match *self {
            Filter::Native(ref field1) => {
                field1.trace(tracer);
            }
            // ...
        }
    }
}
