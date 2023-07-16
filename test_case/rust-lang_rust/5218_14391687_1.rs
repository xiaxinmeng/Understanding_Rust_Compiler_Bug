
#[macro_escape]
mod m {
    macro_rules! test {
        { } => { ~"test" }
    }
}

fn main() {
    io::println(test!());
}
