 rust
#[cfg_attr(target_os = "nacl", main_link_name = "nacl_main")]
fn main() {}

#[cfg(target_os = "nacl")]
#[link(name = "ppapi_cpp", kind = "static")]
#[link(name = "ppapi_simple_cpp", kind = "static")]
#[link(name = "ppapi_stub", kind = "static")]
#[link(name = "cli_main", kind = "static")]
#[link(name = "tar", kind = "static")]
#[link(name = "nacl_spawn", kind = "static")]
extern { }
