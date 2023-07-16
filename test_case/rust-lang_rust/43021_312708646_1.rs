rust
#[derive(Clone, Debug, Serialize,Deserialize)]
pub struct AdjacencyList<N, E> {
    vertex_labels: HashMap<AdjacencyListVertexDescriptor, N>,
    edge_labels: HashMap<AdjacencyListEdgeDescriptor, E>,
    out_edges: HashMap<AdjacencyListVertexDescriptor, Vec<AdjacencyListEdgeDescriptor>>,
    in_edges: HashMap<AdjacencyListVertexDescriptor, Vec<AdjacencyListEdgeDescriptor>>,
    edges: HashMap<AdjacencyListEdgeDescriptor, (AdjacencyListVertexDescriptor, AdjacencyListVertexDescriptor)>,
    next_edge: AdjacencyListEdgeDescriptor,
    next_vertex: AdjacencyListVertexDescriptor,
}
