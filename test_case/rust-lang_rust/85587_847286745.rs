plain
    Checking tracing-subscriber v0.2.16
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0609]: no field `local_def_index` on type `HirOwner`
    --> src/librustdoc/clean/mod.rs:1295:67
     |
1295 |                 self_def_id: Some(DefId::local(qself.hir_id.owner.local_def_index)),
     |
     = note: available fields are: `def_id`
     = note: available fields are: `def_id`
help: one of the expressions' fields has a field of the same name
     |
1295 |                 self_def_id: Some(DefId::local(qself.hir_id.owner.def_id.local_def_index)),

error: aborting due to previous error

For more information about this error, try `rustc --explain E0609`.
