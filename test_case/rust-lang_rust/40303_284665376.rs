rust
struct NodeId {
    owner: DefIndex,
    local_id: LocalNodeId,
}

struct LocalNodeId(u32);
