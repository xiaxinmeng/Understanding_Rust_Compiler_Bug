 rust
struct Ctx<'a,'b> {
    map: &'a mut Map<'b>,
    path: path,
    diag: @span_handler,
}
