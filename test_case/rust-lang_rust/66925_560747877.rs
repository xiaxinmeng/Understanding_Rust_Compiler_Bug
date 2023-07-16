
2019-12-02T20:57:19.8288172Z error[E0308]: try expression alternatives have incompatible types
2019-12-02T20:57:19.8289330Z --> src/tools/miri/src/shims/mod.rs:52:17
2019-12-02T20:57:19.8289702Z |
2019-12-02T20:57:19.8290135Z 52 | Ok(Some(this.load_mir(instance.def, None)?))
2019-12-02T20:57:19.8290620Z | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&rustc::mir::Body<'_>`, found struct `rustc::mir::ReadOnlyBodyCache`
2019-12-02T20:57:19.8290800Z
2019-12-02T20:57:19.9292737Z error[E0308]: try expression alternatives have incompatible types
2019-12-02T20:57:19.9293073Z --> src/tools/miri/src/shims/foreign_items.rs:144:32
2019-12-02T20:57:19.9293295Z |
2019-12-02T20:57:19.9293549Z 144 | return Ok(Some(this.load_mir(start_panic_instance.def, None)?));
2019-12-02T20:57:19.9295202Z | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&rustc::mir::Body<'_>`, found struct `rustc::mir::ReadOnlyBodyCache`
2019-12-02T20:57:19.9295256Z
2019-12-02T20:57:19.9370379Z error[E0308]: try expression alternatives have incompatible types
2019-12-02T20:57:19.9370643Z --> src/tools/miri/src/shims/foreign_items.rs:151:32
2019-12-02T20:57:19.9370844Z |
2019-12-02T20:57:19.9371125Z 151 | return Ok(Some(this.load_mir(panic_impl_instance.def, None)?));
2019-12-02T20:57:19.9371471Z | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `&rustc::mir::Body<'_>`, found struct `rustc::mir::ReadOnlyBodyCache`
2019-12-02T20:57:19.9371528Z
2019-12-02T20:57:21.0463739Z error[E0308]: mismatched types
2019-12-02T20:57:21.0464067Z --> src/tools/miri/src/helpers.rs:142:13
2019-12-02T20:57:21.0464484Z |
2019-12-02T20:57:21.0464717Z 142 | mir,
2019-12-02T20:57:21.0464943Z | ^^^
2019-12-02T20:57:21.0465179Z | |
2019-12-02T20:57:21.0465513Z | expected `&rustc::mir::Body<'_>`, found struct `rustc::mir::ReadOnlyBodyCache`
2019-12-02T20:57:21.0465772Z | help: consider borrowing here: `&mir`
