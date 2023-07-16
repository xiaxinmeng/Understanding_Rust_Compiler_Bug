 rust
extern {
    pub fn manager_node_is_listening_device(manager: *mut Manager, home_id: u32, node_id: u8) -> bool;
    pub fn manager_node_is_frequent_listening_device(manager: *mut Manager, home_id: u32, node_id: u8) -> bool;
    pub fn manager_node_is_beaming_device(manager: *mut Manager, home_id: u32, node_id: u8) -> bool;
}
