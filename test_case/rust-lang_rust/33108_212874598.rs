 rust
fn foo1_error<T>(value: T)
{
    let conv = value.into();
    // ^ error: unable to infer enough type information abou
}

fn foo2_ok<T>(value: T)
{
    let conv: T = value.into();
}

fn foo3_error<T>(value: T)
    where T: Into<String>
{
    let conv: T = value.into();
    // ^ error: mismatched types: expected T, found String
}

fn foo4_ok<T>(value: T)
    where T: Into<String>
{
    let conv = value.into();
}
