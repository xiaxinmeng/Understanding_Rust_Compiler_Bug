
static SLICE: [usize; 1] = [0; 1];

fn ice<'b, 'a: 'b>(_: &'a str) -> impl Iterator<Item=&'a usize> + 'b {
    SLICE.iter()
}
