
// Crate linkage metadata
#[link(name = "thing", vers = "1.0", author = "who")];

// Make a library ("bin" is the default)
#[crate_type = "lib"];

// Load some modules from other files
mod test;
