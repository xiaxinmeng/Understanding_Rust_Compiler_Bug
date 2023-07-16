 rust
.map(|&(def_path_hash, dep_node_index)| { 
             (def_path_hash, self.dep_graph.fingerprint_of(dep_node_index)) 
         }) 
