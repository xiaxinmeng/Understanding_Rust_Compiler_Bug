
fn main() {
    #macro[[#apply[f, [x, ...]], f(x, ...)]];
    fn add(a: int, b: int) -> int { ret a + b; }
    assert (#apply[add, [y, 15]] == 16);
}
