
fn foo(a: Option<u32>, b: Option<u32>) -> bool {
    if let Some(x) = a { true } else { false }
    and
    if let Some(y) = a { true } else { false }
}
