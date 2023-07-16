 rust
struct RunLoop<'a> {
    marker: &'a u8,
}

struct MyContext<'a> {
    rl: &'a mut RunLoop<'a>,
}

fn run<'a>(rl: &'a mut RunLoop<'a>) {
    let mut ctxt = MyContext { rl: rl };
}

fn register(func: fn(&mut RunLoop)) {}

fn main() {
    register(run);
}
