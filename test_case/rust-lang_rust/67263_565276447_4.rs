\n"},"level":"error","spans":[{"file_name":"tests/ui/must_use_candidates.rs","byte_start":849,"byte_end":850,"line_start":51,"line_end":51,"column_start":31,"column_end":32,"is_primary":true,"text":[{"text":"pub fn quoth_the_raven(_more: !) -> u32 {","highlight_start":31,"highlight_end":32}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"for more information, see https://github.com/rust-lang/rust/issues/35121","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"add `#![feature(never_type)]` to the crate attributes to enable","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0658]: The `!` type is experimental\n  --> tests/ui/must_use_candidates.rs:51:31\n   |\nLL | pub fn quoth_the_raven(_more: !) -> u32 {\n   |                               ^\n   |\n   = note: for more information, see https://github.com/rust-lang/rust/issues/35121\n   = help: add `#![feature(never_type)]` to the crate attributes to enable\n\n"}
2019-12-13T01:59:02.8400210Z {"message":"For more information about this error, try `rustc --explain E0658`.","code":null,"level":"failure-note","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0658`.\n"}
2019-12-13T01:59:02.8400338Z 
2019-12-13T01:59:02.8400629Z ------------------------------------------
2019-12-13T01:59:02.8400684Z 
---
2019-12-13T01:59:32.0613897Z normalized stderr:
2019-12-13T01:59:32.0621420Z error[E0425]: cannot find value `x` in this scope
2019-12-13T01:59:32.0622214Z   --> $DIR/result_map_unit_fn_unfixable.rs:16:5
2019-12-13T01:59:32.0622799Z    |
2019-12-13T01:59:32.0622938Z LL |     x.field.map(|value| { do_nothing(value); do_nothing(value) });
2019-12-13T01:59:32.0623049Z    |     ^ not found in this scope
2019-12-13T01:59:32.0623173Z error[E0425]: cannot find value `x` in this scope
2019-12-13T01:59:32.0623533Z   --> $DIR/result_map_unit_fn_unfixable.rs:18:5
2019-12-13T01:59:32.0623848Z    |
2019-12-13T01:59:32.0623848Z    |
2019-12-13T01:59:32.0623932Z LL |     x.field.map(|value| if value > 0 { do_nothing(value); do_nothing(value) });
2019-12-13T01:59:32.0624044Z    |     ^ not found in this scope
2019-12-13T01:59:32.0624162Z error[E0425]: cannot find value `x` in this scope
2019-12-13T01:59:32.0624489Z   --> $DIR/result_map_unit_fn_unfixable.rs:22:5
2019-12-13T01:59:32.0624584Z    |
2019-12-13T01:59:32.0624584Z    |
2019-12-13T01:59:32.0624648Z LL |     x.field.map(|value| {
2019-12-13T01:59:32.0624738Z    |     ^ not found in this scope
2019-12-13T01:59:32.0624872Z error[E0425]: cannot find value `x` in this scope
2019-12-13T01:59:32.0625250Z   --> $DIR/result_map_unit_fn_unfixable.rs:26:5
2019-12-13T01:59:32.0625353Z    |
2019-12-13T01:59:32.0625353Z    |
2019-12-13T01:59:32.0625431Z LL |     x.field.map(|value| { do_nothing(value); do_nothing(value); });
2019-12-13T01:59:32.0625536Z    |     ^ not found in this scope
2019-12-13T01:59:32.0625666Z error[E0658]: The `!` type is experimental
2019-12-13T01:59:32.0625971Z   --> $DIR/result_map_unit_fn_unfixable.rs:29:21
2019-12-13T01:59:32.0626066Z    |
2019-12-13T01:59:32.0626066Z    |
2019-12-13T01:59:32.0626139Z LL |     let res: Result<!, usize> = Ok(42).map(diverge);
2019-12-13T01:59:32.0626306Z    |
2019-12-13T01:59:32.0626704Z    = note: for more information, see https://github.com/rust-lang/rust/issues/35121
2019-12-13T01:59:32.0626704Z    = note: for more information, see https://github.com/rust-lang/rust/issues/35121
2019-12-13T01:59:32.0626829Z    = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-13T01:59:32.0626966Z error: aborting due to 5 previous errors
2019-12-13T01:59:32.0627035Z 
2019-12-13T01:59:32.0627120Z Some errors have detailed explanations: E0425, E0658.
2019-12-13T01:59:32.0627463Z For more information about an error, try `rustc --explain E0425`.
2019-12-13T01:59:32.0627463Z For more information about an error, try `rustc --explain E0425`.
2019-12-13T01:59:32.0633997Z 
2019-12-13T01:59:32.0645596Z 
2019-12-13T01:59:32.0649308Z expected stderr:
2019-12-13T01:59:32.0657515Z error[E0425]: cannot find value `x` in this scope
2019-12-13T01:59:32.0658556Z   --> $DIR/result_map_unit_fn_unfixable.rs:16:5
2019-12-13T01:59:32.0659044Z    |
2019-12-13T01:59:32.0659253Z LL |     x.field.map(|value| { do_nothing(value); do_nothing(value) });
2019-12-13T01:59:32.0659472Z    |     ^ not found in this scope
2019-12-13T01:59:32.0659836Z error[E0425]: cannot find value `x` in this scope
2019-12-13T01:59:32.0660374Z   --> $DIR/result_map_unit_fn_unfixable.rs:18:5
2019-12-13T01:59:32.0660610Z    |
2019-12-13T01:59:32.0660610Z    |
2019-12-13T01:59:32.0660825Z LL |     x.field.map(|value| if value > 0 { do_nothing(value); do_nothing(value) });
2019-12-13T01:59:32.0661033Z    |     ^ not found in this scope
2019-12-13T01:59:32.0661374Z error[E0425]: cannot find value `x` in this scope
2019-12-13T01:59:32.0661859Z   --> $DIR/result_map_unit_fn_unfixable.rs:22:5
2019-12-13T01:59:32.0662089Z    |
2019-12-13T01:59:32.0662089Z    |
2019-12-13T01:59:32.0662262Z LL |     x.field.map(|value| {
2019-12-13T01:59:32.0662461Z    |     ^ not found in this scope
2019-12-13T01:59:32.0663136Z error[E0425]: cannot find value `x` in this scope
2019-12-13T01:59:32.0663632Z   --> $DIR/result_map_unit_fn_unfixable.rs:26:5
2019-12-13T01:59:32.0663866Z    |
2019-12-13T01:59:32.0663866Z    |
2019-12-13T01:59:32.0664055Z LL |     x.field.map(|value| { do_nothing(value); do_nothing(value); });
2019-12-13T01:59:32.0664270Z    |     ^ not found in this scope
2019-12-13T01:59:32.0664634Z error: aborting due to 4 previous errors
2019-12-13T01:59:32.0664781Z 
2019-12-13T01:59:32.0665243Z For more information about this error, try `rustc --explain E0425`.
2019-12-13T01:59:32.0665470Z 
2019-12-13T01:59:32.0665470Z 
2019-12-13T01:59:32.0665613Z 
2019-12-13T01:59:32.0665812Z diff of stderr:
2019-12-13T01:59:32.0665956Z 
2019-12-13T01:59:32.0666156Z  error[E0425]: cannot find value `x` in this scope
2019-12-13T01:59:32.0666773Z    --> $DIR/result_map_unit_fn_unfixable.rs:16:5
2019-12-13T01:59:32.0667177Z     |
2019-12-13T01:59:32.0667354Z  LL |     x.field.map(|value| { do_nothing(value); do_nothing(value) });
2019-12-13T01:59:32.0667762Z     |     ^ not found in this scope
2019-12-13T01:59:32.0668118Z  error[E0425]: cannot find value `x` in this scope
2019-12-13T01:59:32.0668758Z    --> $DIR/result_map_unit_fn_unfixable.rs:18:5
2019-12-13T01:59:32.0668984Z     |
2019-12-13T01:59:32.0668984Z     |
2019-12-13T01:59:32.0669187Z  LL |     x.field.map(|value| if value > 0 { do_nothing(value); do_nothing(value) });
2019-12-13T01:59:32.0669564Z     |     ^ not found in this scope
2019-12-13T01:59:32.0820168Z  error[E0425]: cannot find value `x` in this scope
2019-12-13T01:59:32.0820903Z    --> $DIR/result_map_unit_fn_unfixable.rs:22:5
2019-12-13T01:59:32.0820994Z     |
2019-12-13T01:59:32.0820994Z     |
2019-12-13T01:59:32.0821339Z  LL |     x.field.map(|value| {
2019-12-13T01:59:32.0821434Z     |     ^ not found in this scope
2019-12-13T01:59:32.0821593Z  error[E0425]: cannot find value `x` in this scope
2019-12-13T01:59:32.0822086Z    --> $DIR/result_map_unit_fn_unfixable.rs:26:5
2019-12-13T01:59:32.0822177Z     |
2019-12-13T01:59:32.0822177Z     |
2019-12-13T01:59:32.0822271Z  LL |     x.field.map(|value| { do_nothing(value); do_nothing(value); });
2019-12-13T01:59:32.0822375Z     |     ^ not found in this scope
2019-12-13T01:59:32.0823239Z -error: aborting due to 4 previous errors
2019-12-13T01:59:32.0823326Z +error[E0658]: The `!` type is experimental
2019-12-13T01:59:32.0823610Z +  --> $DIR/result_map_unit_fn_unfixable.rs:29:21
2019-12-13T01:59:32.0823686Z +   |
2019-12-13T01:59:32.0823686Z +   |
2019-12-13T01:59:32.0823776Z +LL |     let res: Result<!, usize> = Ok(42).map(diverge);
2019-12-13T01:59:32.0823942Z +   |
2019-12-13T01:59:32.0824267Z +   = note: for more information, see https://github.com/rust-lang/rust/issues/35121
2019-12-13T01:59:32.0824267Z +   = note: for more information, see https://github.com/rust-lang/rust/issues/35121
2019-12-13T01:59:32.0824377Z +   = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-13T01:59:32.0824753Z -For more information about this error, try `rustc --explain E0425`.
2019-12-13T01:59:32.0824861Z +error: aborting due to 5 previous errors
2019-12-13T01:59:32.0824943Z +
2019-12-13T01:59:32.0825033Z +Some errors have detailed explanations: E0425, E0658.
---
2019-12-13T01:59:32.0831161Z 
2019-12-13T01:59:32.0831726Z ------------------------------------------
2019-12-13T01:59:32.0831799Z stderr:
2019-12-13T01:59:32.0832042Z ------------------------------------------
2019-12-13T01:59:32.0835319Z {"message":"cannot find value `x` in this scope","code":{"code":"E0425","explanation":"An unresolved name was used.\n\nErroneous code examples:\n\n