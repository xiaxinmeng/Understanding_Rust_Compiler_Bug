
error[E0015]: calls in constants are limited to constant functions, struct and enum constructors
  --> src/messages/protocol.rs:41:37
   |
41 | pub const CONNECT_MESSAGE_ID: u16 = Connect::message_id();
   |                                     ^^^^^^^^^^^^^^^^^^^^^

