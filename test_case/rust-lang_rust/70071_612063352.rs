
use std::{io, fs, env};

let arg = env::args().nth(1).unwrap_or("-".into());

// We only declare, not initialize the values.
let (mut stdin, mut file);
