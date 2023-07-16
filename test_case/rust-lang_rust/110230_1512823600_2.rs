rust
#[derive(Default)]
pub struct SpanInterner {
    spans: FxIndexSet<SpanData>,
}
