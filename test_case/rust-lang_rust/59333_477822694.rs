rust
struct Runner;

type RuntimeImpl = Runner;

trait Runtime {
    fn run(&self) {}
}

impl Runtime for RuntimeImpl {}

fn main() {
    Runner.run();
}
