 rust
macro_rules! node_getters {
    ( $($name: ident -> $t: ty),+ ) => {
        $(pub fn $name(manager: *mut Manager, home_id: u32, node_id: u8) -> $t;)*
    }
}

extern {
  node_getters! {
    manager_node_is_listening_device -> bool,
    manager_node_is_frequent_listening_device -> bool,
    manager_node_is_beaming_device -> bool
  }
}
