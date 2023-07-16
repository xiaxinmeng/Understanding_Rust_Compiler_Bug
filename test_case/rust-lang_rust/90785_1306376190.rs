rust
eprintln!("Something something");
stderr.write_all(tty_escape_cursor_up_clear_line).unwrap();
eprintln!("Something else");
