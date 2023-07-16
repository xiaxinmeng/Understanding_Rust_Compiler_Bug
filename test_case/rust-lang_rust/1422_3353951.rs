
fn count_zeroes(x: [int]) -> uint {
    let cnt = 0u;
    vec::iter(x, {|elt| if (elt == 0) { cnt += 1u; }});
    ret cnt;
}
