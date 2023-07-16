
#[derive(Default)]
struct RR<'a> {
    r: Cell<Option<&'a RR<'a>>>,
}
