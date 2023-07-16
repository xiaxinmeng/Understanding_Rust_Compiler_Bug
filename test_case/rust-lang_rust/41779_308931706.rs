rust
impl Flags {
    pub fn parse(args: &[String]) -> Flags {
// [...]
        // All subcommands can have an optional "Available paths" section
        if matches.opt_present("verbose") {
            let flags = Flags::parse(&["build".to_string()]);
            let mut config = Config::default();
            config.build = flags.build.clone();
            let mut build = Build::new(flags, config);
            metadata::build(&mut build);
            let maybe_rules_help = step::build_rules(&build).get_help(subcommand);
            if maybe_rules_help.is_some() {
                extra_help.push_str(maybe_rules_help.unwrap().as_str());
            }
        } else {
            extra_help.push_str(format!("Run `./x.py {} -h -v` to see a list of available paths.",
                     subcommand).as_str());
        }
// [...]
    }
}
