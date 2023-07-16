
use core::run;

fn main() {
    let output;
    {
        let mut p = run::Process::new("cat", [], run::ProcessOptions::new());
        output = p.output();
    }
    output.read_byte();
}
