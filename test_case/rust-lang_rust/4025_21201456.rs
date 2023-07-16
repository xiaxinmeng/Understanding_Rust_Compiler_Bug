
fn print<'a,'b>(b: bool, s1: &'a str, s2: &'b str) {
    println(if b { s1 } else { s2 });
}

fn main() {}
