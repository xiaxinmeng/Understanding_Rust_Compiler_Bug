
····let arguments = application.get_matches();

····match arguments.subcommand() {
········("host", Some(sub_arguments)) => host::process_matches(sub_arguments),
····↦_·=>↦{
············application.print_help();
········}
····}
