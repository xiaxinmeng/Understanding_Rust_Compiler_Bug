 rust
enum StdioContainer {
    CreatePipe(bool)
}

struct Test<'a> {
    args: &'a [~str],
    io: &'a [StdioContainer]
}

fn main() {
    let test = Test {
        args: &[],
        io: &[CreatePipe(true)]
    };
}
