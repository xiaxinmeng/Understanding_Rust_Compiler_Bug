
// Uses a JSON file to store configuration options.
// Demonstrates a multi-file crate, command line parsing, pattern matching, and file IO.
// To run the executable: rustc -o json-config crate.rc && ./json-config -v

// Note that main in rust does not return an error code. If you want to return
// a non-zero exit code then use libc::exit.
fn main(args: ~[~str])
{
    let options = options::parse_command_line(args);
    io::println(fmt!("Working for %s", options.user));

    if options.verbose
    {
        io::println(fmt!("Options: %?", options));
    }
}
