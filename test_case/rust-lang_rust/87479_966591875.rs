
error: Missing required bounds on SerializeUnit
  --> diny_core/src/backend/format.rs:54:5
   |
54 |     type SerializeUnit<'w, W>: Future<Output=Result<(), Self::Error>> + Unpin where W: 'w + io::AsyncWrite + Unpin;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-
   |                                                                                                                   |
   |                                                                                                                   help: add the required where clauses: `, Self: 'w`
