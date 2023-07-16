rust
for &[x, y] in parameters.exact_chunks(2) {
    builder.svg_event(SvgEvent::MoveTo(TypedPoint2D::new(x, y)));
}
