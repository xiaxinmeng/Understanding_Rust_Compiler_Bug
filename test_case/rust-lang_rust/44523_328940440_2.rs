rust
const OTHER_INDICES_LEN: usize = MAX_PORTS.v as usize;

pub struct RoutingTableEntry {
    other_indices: [TableIndex; OTHER_INDICES_LEN],
}
