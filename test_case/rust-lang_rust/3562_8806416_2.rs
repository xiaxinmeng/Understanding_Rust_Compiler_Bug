
use json = std::json;
use Json = std::json::Json;
use std::map::{HashMap};
use io::WriterUtil;
use std::getopts::*;

// Various options used to control the behavior of the exe.
struct Options
{
    // these are from the config file
    user: ~str,
    poll_rate: float,

    // these are from the command line
    verbose: bool,
}

// Parses the command line, reads in a json config file, and returns the results
// using an Options struct.
#[allow(implicit_copies)]    // args is full of non-implicitly copyable
#[allow(non_implicitly_copyable_typarams)]
fn parse_command_line(args: ~[~str]) -> Options
{
    // It's good practice to do this before invoking getopts because getopts
    // will fail if a required option is missing (although at the moment all
    // of our arguments are optional).
    if args.contains(~"-h") || args.contains(~"--help")
    {
        print_usage();
        libc::exit(0);
    }

    let opts = ~[
        optopt(~"config"),        // *opt's take an optional argument
        optflag(~"h"),            // *flag's take no arguments
        optflag(~"help"),
        optflag(~"v"),
        optflag(~"verbose"),
        optflag(~"V"),
        optflag(~"version"),
    ];
    let matched = match getopts(vec::tail(args), opts)   // first argument is the exe name
    {
        result::Ok(copy m) => {m}
        result::Err(ref f) => {io::stderr().write_line(fail_str(*f)); libc::exit(1)}
    };
    if opt_present(matched, ~"version")
    {
        io::println(fmt!("json-config %s", exe_version));
        libc::exit(0);
    }
    else if matched.free.len() != 0
    {
        io::stderr().write_line(fmt!("Unexpected positional argument(s): %?", matched.free));
        libc::exit(1);
    }

    let config = if opt_present(matched, ~"config") {opt_str(matched, ~"config")} else {~"config.json"};
    let options = Options
    {
        user: ~"",
        poll_rate: 0.0,
        verbose: opt_present(matched, ~"v") || opt_present(matched, ~"verbose"),
    };
    let options = add_config(&path::Path(config), &options);
    validate_options(&options);
    move options
}

// https://github.com/mozilla/rust/issues/3567
priv const exe_version: &str = "1.0";

// Items currently default to public (package and external visibility).
// Private items are private to their module (these semantics may change in the future).
priv fn validate_options(config: &Options)
{
    if config.user.is_empty()
    {
        io::stderr().write_line("user isn't set");
        libc::exit(1);
    }
    if config.poll_rate <= 0.0
    {
        io::stderr().write_line(fmt!("poll_rate should be positive but is %f", config.poll_rate));
        libc::exit(1);
    }
}

priv fn print_usage()
{
    io::println(fmt!("json-config %s - rust sample app", exe_version));
    io::println("");
    io::println("./json-config [options]");
    io::println("--config=FILE  use a custom JSON config file (defaults to config.json)");
    io::println("-h, --help     print this message and exits");
    io::println("-v, --verbose  enable extra output");
    io::println("--version      print the version number and exits");
}

// Add informatiom from the config file to config and return a new config.
priv fn add_config(path: &Path, config: &Options) -> Options
{
    // Comments are not part of the JSON spec, but they are awfully nice
    // to have in config files so we'll pre-process the json files and strip
    // them out ourselves.
    fn read_json(reader: io::Reader) -> ~str
    {
        let mut contents = ~"";

        for reader.each_line
        |line|
        {
            if !line.trim_left().starts_with("#")
            {
                contents += line;
            }
        }

        return contents;
    }

    match io::file_reader(path)
    {
        result::Ok(reader) =>
        {
            match json::from_str(read_json(reader))
            {
                result::Ok(std::json::Dict(data)) =>
                {
                    move Options
                    {
                        user: get_config_str(path, data, ~"user"),
                        poll_rate: get_config_float(path, data, ~"poll_rate"),
                        .. *config   // this is the functional update syntax for structs
                    }
                }
                result::Ok(x) =>
                {
                    io::stderr().write_line(fmt!("Error parsing '%s': expected json::Dict but found %?.", path.to_str(), x));
                    libc::exit(1)
                }
                result::Err(err) =>
                {
                    io::stderr().write_line(fmt!("Error parsing '%s' on line %?: %s.", path.to_str(), err.line, *err.msg));
                    libc::exit(1)
                }
            }
        }
        result::Err(ref err) =>
        {
            io::stderr().write_line(fmt!("Error reading '%s': %s.", path.to_str(), *err));
            libc::exit(1)
        }
    }
}

// Search the json for a String named key.
#[allow(implicit_copies)]    // json needs to stop using ~str, see https://github.com/mozilla/rust/issues/3350
#[allow(non_implicitly_copyable_typarams)]
priv fn get_config_str(path: &Path, data: HashMap<~str, Json>, key: ~str) -> ~str
{
    match data.find(key)
    {
        option::Some(json::String(value)) =>
        {
            copy *value
        }
        option::Some(x) =>
        {
            io::stderr().write_line(fmt!("In '%s' %s was expected to be a json::String but was %?.", path.to_str(), key, x));
            libc::exit(1)
        }
        option::None =>
        {
            io::stderr().write_line(fmt!("Expected to find %s in '%s'.", key, path.to_str()));
            libc::exit(1)
        }
    }
}

// Search the json for a Num named key.
#[allow(implicit_copies)]
#[allow(non_implicitly_copyable_typarams)]
priv fn get_config_float(path: &Path, data: HashMap<~str, Json>, key: ~str) -> float
{
    match data.find(key)
    {
        option::Some(json::Num(value)) =>
        {
            value
        }
        option::Some(x) =>
        {
            io::stderr().write_line(fmt!("In '%s' %s was expected to be a json::Num but was %?.", path.to_str(), key, x));
            libc::exit(1)
        }
        option::None =>
        {
            io::stderr().write_line(fmt!("Expected to find %s in '%s'.", key, path.to_str()));
            libc::exit(1)
        }
    }
}
