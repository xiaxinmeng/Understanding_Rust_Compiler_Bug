rust
fn _v_closure<'a: 'b, 'b>(_: &'b (), _: &'a ()) {
    let _v_lambda = |v: &'a Vec<String>| -> Vec<&'b String> {
        v.iter().chain(v.iter()).collect::<Vec<&String>>()
    };
}
