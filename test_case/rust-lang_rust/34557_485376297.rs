rust
use regex::Regex;

let re = Regex::new( r"\w+::" ).unwrap();

let s = re.replace_all( name, "" );
