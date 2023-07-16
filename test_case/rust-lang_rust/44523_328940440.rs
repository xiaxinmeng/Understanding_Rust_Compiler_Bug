rust
pub struct RoutingTableEntry {
    index: TableIndex,
    uuid: Uuid,
    may_send: bool,
    inuse: bool,
    parent: PortNo,
    mask: Mask,
    other_indices: [TableIndex; MAX_PORTS.v as usize],
}
