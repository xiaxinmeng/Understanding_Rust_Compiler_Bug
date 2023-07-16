rust
buf += f"hello {name}";
// Equivalent to today's
buf += format_args!("hello {name}", name = name);
