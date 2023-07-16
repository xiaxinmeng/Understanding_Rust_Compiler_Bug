 rust
#![feature(plugin)]
#![plugin(docopt_macros)]

extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

docopt!(Args derive Debug, "
Usage: foo (--help | --version)

Options:
    -h, --help     Show this message
    --version      Show the version of foo
");

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());
    println!("{:?}", args);
}
