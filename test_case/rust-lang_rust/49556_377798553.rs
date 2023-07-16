
error[E0283]: type annotations required: cannot resolve `_: serde::Serialize`
  --> src/context/mod.rs:54:13
   |
54 |             DataSet::from_block_storage(
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
note: required by `<resource::DataSet<T>>::from_block_storage`
  --> src/resource/mod.rs:63:5
   |
63 | /     pub fn from_block_storage(
64 | |         manager: &Arc<BlockManager>,
65 | |         server_id: u64,
66 | |         task: UUID,
...  |
75 | |             }))
76 | |     }
   | |_____^
