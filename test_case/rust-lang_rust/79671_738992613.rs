rust
use std::env;
use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let output_file_name = env::args().skip(1).nth(0).expect("Missing output file name arg");
    let mut output = File::create(output_file_name)?;

    writeln!(output, "pub struct Lol {{")?;
    writeln!(output, "pub i: bool,")?;
    writeln!(output, "pub o: bool,")?;
    writeln!(output, "}}")?;

    // Uncomment to ignore `liveness_and_intrinsic_checking` etc
    //writeln!(output, "#[automatically_derived]")?;
    writeln!(output, "impl Lol {{")?;
    writeln!(output, "pub fn prop(&mut self) {{")?;
    // Modify to see mem explosion in `liveness_and_intrinsic_checking` and nonlinear time scaling in `item_bodies_checking`
    let count = 30000;
    let mut last_temp_name = None;
    for i in 0..count {
        let temp_name = format!("temp{}", i);

        writeln!(output, "let {} = !{};", temp_name, last_temp_name.unwrap_or("self.i".into()))?;

        last_temp_name = Some(temp_name);
    }
    writeln!(output, "self.o = {};", last_temp_name.unwrap())?;
    writeln!(output, "}}")?;
    writeln!(output, "}}")?;

    Ok(())
}
