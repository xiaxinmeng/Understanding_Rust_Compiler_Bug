
mod a {
    const A: i32 = 3;
    macro a() { A }
}
mod b {
    const B: i32 = 2;
    macro b() { B }
}
mod sum {
    macro sum($lhs, $rhs) { $lhs!() + $rhs!() }
}
fn whatami() -> i32 {
    use a::a;
    use b::b;
    use sum::sum;
    sum!(a, b)
}
