
// Exe which uses the sxml library.
// Demonstrates linking against and using a custom rust library.
// To run the executable: cd xml-client && rustc -L ../sxml --test client.rc && ./client
#[link(name = "client", vers = "0.2", uuid = "D71AEFEB-650C-46E5-91C3-36E9406AEE8E")];

#[author = "Jesse Jones"];
#[license = "MIT"];

#[forbid(unused_imports)];
#[forbid(implicit_copies)];
#[forbid(deprecated_pattern)];
#[allow(structural_records)];   // TODO: enable some of these
#[allow(deprecated_mode)];
#[allow(non_implicitly_copyable_typarams)];

// Tell rustc which libraries to link in.
extern mod std;
extern mod sxml (name = "sxml", vers = "1.0");

mod client;

