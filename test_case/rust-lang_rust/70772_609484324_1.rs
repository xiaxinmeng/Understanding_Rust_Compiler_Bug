
let mut param_name = vec![];
s.read_while(&mut param_name, |b| !matches!(b, b';' | b'='))?;
