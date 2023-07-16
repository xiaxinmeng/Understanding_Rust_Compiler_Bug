`
   --> tools\rls\src\build\plan.rs:189:80
    |
189 |                 *target.kind() == TargetKind::CustomBuild && target.src_path().is_path()
    |                                                                                ^^^^^^^
error[E0599]: no method named `path` found for type `&std::path::Path` in the current scope
   --> tools\rls\src\build\plan.rs:191:56
    |
191 |             .map(|(key, unit)| (unit.target.src_path().path(), key.clone()))
    |                                                        ^^^^
error[E0599]: no method named `path` found for type `&std::path::Path` in the current scope
   --> tools\rls\src\build\plan.rs:202:26
    |
202 |                         .path()
    |                          ^^^^
error[E0599]: no method named `path` found for type `&std::path::Path` in the current scope
  --> tools\rls\src\project_model.rs:82:44
   |
82 |                     .map(|t| (t.src_path().path().to_owned(), t.name().replace('-', "_"))),
   |                                            ^^^^
error: aborting due to 4 previous errors
For more information about this error, try `rustc --explain E0599`.
[RUSTC-TIMING] rls test:false 4.825
error: Could not compile `rls`.
