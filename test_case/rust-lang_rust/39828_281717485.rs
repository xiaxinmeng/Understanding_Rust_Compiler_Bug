
lunch-box. rustc ~/tmp/foo.rs -Zincremental=$PWD/incr -Zincremental-info
incremental: 9840 nodes in dep-graph
incremental: 13341 edges in dep-graph
incremental: 0 edges in serialized dep-graph
incremental: 0 hashes in serialized dep-graph
incremental: re-using 0 out of 1 modules
lunch-box. rustc ~/tmp/foo.rs -Zincremental=$PWD/incr -Zincremental-info
incremental: session directory: 4 files hard-linked
incremental: session directory: 0 files copied
incremental: 9840 nodes in dep-graph
incremental: 13341 edges in dep-graph
incremental: 0 edges in serialized dep-graph
incremental: 0 hashes in serialized dep-graph
incremental: re-using 0 out of 1 modules
lunch-box. cat ~/tmp/foo.rs
#![crate_type="rlib"]
pub fn foo<T>() { }
