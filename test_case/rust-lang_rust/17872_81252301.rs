 rust
#![feature(rustc_private)]

extern crate graphviz;

use std::borrow::IntoCow;
use graphviz as dot;

type Nd = usize;
type Ed = (usize, usize);
struct Graph<'a> {
    nodes: Vec<&'a str>,
    edges: Vec<Ed>,
}

impl<'a> dot::Labeller<'a, Nd, Ed> for Graph<'a> {
    fn graph_id(&self) -> dot::Id {
        dot::Id::new("example2").unwrap()
    }

    fn node_id(&self, n: &Nd) -> dot::Id {
        dot::Id::new(format!("N{}", n)).unwrap()
    }

    fn node_label<'b>(&self, n: &Nd) -> dot::LabelText<'b> {
        dot::LabelText::LabelStr(self.nodes[*n].into_cow())
        // error: cannot infer an appropriate lifetime for autoref due to conflicting requirements
        // help: consider using an explicit lifetime parameter as shown: fn node_label(&self, n: &Nd) -> dot::LabelText<'a>
    }
}

fn main() { }
