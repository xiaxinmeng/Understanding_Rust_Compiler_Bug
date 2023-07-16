
fn main() {
    let v1 = { if 1 > 2 {2} else {3} + 5 };   // does not parse, unexpected token at +
    let v2 =   if 1 > 2 {2} else {3} + 5 ;    // parses and evaluates as expected
}
