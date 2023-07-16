rust
#[cfg(host_family = "windows")]
macro_rules! PATH_SEPARATOR {() => (
    r"\"
)}
#[cfg(not(host_family = "windows"))]
macro_rules! PATH_SEPARATOR {() => (
    r"/"
)}

include_bytes!(concat!(
    env!("OUT_DIR"), PATH_SEPARATOR!(), "my_file"
));
