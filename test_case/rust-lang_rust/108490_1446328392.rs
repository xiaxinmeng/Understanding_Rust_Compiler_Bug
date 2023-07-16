
#[derive(Default)]
struct RR<'a, 'b> {
    r: Option<&'a mut &'b i32>,
}
