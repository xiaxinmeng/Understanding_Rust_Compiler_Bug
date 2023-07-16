
// Multi file crates use an rc file to tell the compiler about various pieces of
// metadata and about which modules are part of the crate.

// This is the required bit of meta-data for a crate.
// The uuid should be generated using uuidgen (or the equivalent).
#[link(name = "json-config", vers = "1.0", uuid = "60B3CC26-037D-443A-891A-FB265232BE8D")];

// These attributes are used by rustdoc and rustc --ls.
#[author = "Jesse Jones"];
#[license = "MIT"];
#[doc = "Multi-file crate exe example"];

// These control various compiler lint passes. You can see them all by doing
// `rustc -W help`.
#[warn(unused_imports)];
#[warn(deprecated_pattern)];

// Link in the rust std library.
extern mod std;

// List our modules. When this crate is compiled the files with the same names
// will be automatically compiled into the crate.
mod main;
mod options;
