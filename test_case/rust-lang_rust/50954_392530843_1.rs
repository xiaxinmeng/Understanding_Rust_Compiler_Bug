
May 25 17:33:44.418 INFO kablam! error[E0283]: type annotations required: cannot resolve `std::string::String: std::convert::AsRef<_>`
May 25 17:33:44.418 INFO kablam!    --> src/websocket/types/mod.rs:163:76
May 25 17:33:44.419 INFO kablam!     |
May 25 17:33:44.419 INFO kablam! 163 |             } => Channel::room_map_view(room_name, shard_name.as_ref().map(AsRef::as_ref)),
May 25 17:33:44.419 INFO kablam!     |                                                                            ^^^^^^^^^^^^^
May 25 17:33:44.419 INFO kablam!     |
May 25 17:33:44.419 INFO kablam!     = note: required by `std::convert::AsRef::as_ref`
