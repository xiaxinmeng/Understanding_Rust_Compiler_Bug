rust
fn _v_closure<'a, 'b>(_: &'b (), _: &'a ())
                     -> impl Fn(&'a Vec<String>) -> Vec<&'b String>
    where 'a: 'b
{
    |v| { v.iter().chain(v.iter()).collect::<Vec<&String>>() }
}
