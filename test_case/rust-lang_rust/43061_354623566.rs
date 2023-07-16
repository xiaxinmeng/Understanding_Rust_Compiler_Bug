rust
// error: internal compiler error: broken MIR in NodeId(4) (_2 = ()):
//     bad assignment (! = ()): Sorts(ExpectedFound { expected: !, found: () })
#![crate_type = "lib"]
#![feature(never_type)]

fn blah(x: !) {
    x;
    // () // Uncommenting this explicit unit return avoids the ICE
}
