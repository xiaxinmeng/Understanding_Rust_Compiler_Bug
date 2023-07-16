rust
#![warn(rust_2021_incompatible_closure_captures)]

pub fn graph(activations: Vec<String>) -> im_rc::OrdMap<String, String> {
    let mut graph = im_rc::OrdMap::new();
    activations.iter().for_each(|r| {
        graph.entry(r.clone()).or_insert_with(String::new);
    });
    graph
}
