rust
#![feature(rustc_attrs)]

#[rustc_mir(borrowck_graphviz_preflow="preflow.dot",
            borrowck_graphviz_postflow="postflow.dot")]
