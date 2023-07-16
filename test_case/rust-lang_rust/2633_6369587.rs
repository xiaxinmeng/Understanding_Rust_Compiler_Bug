
fn a_val(++x: @int, +y: @int) -> int {
    free(y);
    #debug["deref"];
    *x
}

fn free(-x: @int) {}

fn main() {
    let z = @22;
    assert 22 == a_val(z, z);
}
