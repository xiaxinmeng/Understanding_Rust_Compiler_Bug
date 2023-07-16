rust
#[no_mangle]
pub extern fn rsvg_node_is_same (raw_node1: *const RsvgNode, raw_node2: *const RsvgNode) -> bool {
    match (raw_node1.as_ref(), raw_node2.as_ref()) {
        (Some(node1), Some(node2)) => same_node(&**node1, &**node2),
        (None, None) => true,
        _ => false
    }
}
