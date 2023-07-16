
pub type RsvgNode = Rc<Node>;

#[no_mangle]
pub extern fn rsvg_node_is_same (raw_node1: *const RsvgNode, raw_node2: *const RsvgNode) -> bool {
    if raw_node1.is_null () && raw_node2.is_null () {
        true
    } else if !raw_node1.is_null () && !raw_node2.is_null () {
        let node1: &RsvgNode = unsafe { & *raw_node1 };
        let node2: &RsvgNode = unsafe { & *raw_node2 };

        Rc::ptr_eq (node1, node2)
    } else {
        false
    }
}
