
fn foo1(t: &'a str) -> &'a str { // 'a is not defined
    t
}

fn foo4<'a>(t: &'a str)-> &'b str { // 'b is not defined
    t
}
