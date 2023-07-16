
error[E0495]: cannot infer an appropriate lifetime for lifetime parameter `'a` due to conflicting requirements
  --> src/msgmgr/mod.rs:9:10
   |
9  | #[derive(Clone)]
   |          ^^^^^
   |
note: first, the lifetime cannot outlive the lifetime `'a` as defined on the impl at 10:19...
  --> src/msgmgr/mod.rs:10:19
   |
10 | pub struct RmbMsg<'a> {
   |                   ^^
   = note: ...so that the types are compatible:
           expected msgmgr::RmbMsg<'a>
              found msgmgr::RmbMsg<'_>
   = note: but, the lifetime must be valid for the static lifetime...
   = note: ...so that the types are compatible:
           expected std::clone::Clone
              found std::clone::Clone
