
pub struct SpanHandle {
    Id(usize),
    CallSite,
    Parent(Box<SpanHandle>),
    // ... other values returned by functions
}
