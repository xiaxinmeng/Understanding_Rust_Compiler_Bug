rust
// Arguments before the first '--' are filters, except if they start with `-` which are probably
// meant to be flags, and we don't support any flags.
for arg in args.as_ref().take_while(|arg| arg != "--") {
  if arg.starts_with("-") {
    eprintln!("Unknown command-line flag {arg}");
  }
  filters.push(arg);
}
// Append remaining arguments as filters
filters.extend(args);
