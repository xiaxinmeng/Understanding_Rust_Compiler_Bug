rust
type OtherIndices = [TableIndex; MAX_PORTS.v as usize];

pub struct RoutingTableEntry {
    other_indices: OtherIndices,
}
