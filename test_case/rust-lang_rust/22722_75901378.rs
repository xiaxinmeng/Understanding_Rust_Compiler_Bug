 rust
// ...
fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("{}", VERSION);
        return
    }

    match server::start(&args.flag_listen) {
        Ok(_) => println!("listening on {} ...", &args.flag_listen),
        Err(err) => panic!("failed to start hearts server: {}", err),
    }
}
