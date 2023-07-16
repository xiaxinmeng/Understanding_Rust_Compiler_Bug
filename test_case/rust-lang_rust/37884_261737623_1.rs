 rust
error[E0491]: in type `&'static std::sync::Arc<std::sync::RwLock<tcp::TCP<'_>>>`, reference has a longer lifetime than the data it references
   --> src/main.rs:106:1
    |
106 | pub fn accept_cmd(tcp_ctx: &'static Arc<RwLock<TCP>>, dl_ctx: &Arc<RwLock<DataLink>>, port: u16) {
    | ^
    |
    = note: the pointer is valid for the static lifetime
    = note: but the referenced data is only valid for the anonymous lifetime #1 defined on unknown free region bounded by scope CodeExtent(9898/CallSiteScope { fn_id: NodeId(4519), body_id: NodeId(18427) })
