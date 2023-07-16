rust
// doesn't compile
.constraints(vec![Constraint::Percentage(10)].as_ref())
// compiles
.constraints(vec![Constraint::Percentage(10)].as_slice())
.constraints(vec![Constraint::Percentage(10)])
.constraints([Constraint::Percentage(10)].as_ref())
.constraints(&[Constraint::Percentage(10)])
.constraints([Constraint::Percentage(10)])
