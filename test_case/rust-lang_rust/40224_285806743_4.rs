rust
fn ok_trace1() {
    return;
    let _x = Some(return); // _x has type Option<?T> where ?T is unconstrained
}
