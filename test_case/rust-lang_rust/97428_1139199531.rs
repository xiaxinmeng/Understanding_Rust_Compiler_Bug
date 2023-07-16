rust
#[derive(Default)]
pub struct Run {
    pub a: Option<String>,
}

impl Run {
    #[inline(never)]
    pub fn run(&mut self) -> ! {
        self.run()
    }
}

pub fn foo() -> Run {
    let mut run = Run::default();
    run.run();
    run
}

fn main() {
    foo();
}
