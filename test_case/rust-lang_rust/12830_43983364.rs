 .rs
match chars.next() {
    '/' => { self.state = EndTagOpen; }
    c if match c.to_ascii_opt() {
        Some(a) => { do_something(a.to_lower()); true }
        _ => false,
    } => (),
    c if other_condition => { ... }
    _ => parse_error()
