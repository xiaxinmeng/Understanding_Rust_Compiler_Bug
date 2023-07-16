 rust
match Command::new("dsymutil").arg(out_filename).stdout(io::process::Ignored)
                              .stderr(io::process::Ignored).status() {
    Ok(..) => {}
    ...
