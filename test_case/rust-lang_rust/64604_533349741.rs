
2019-09-19T18:06:26.4266026Z error[E0433]: failed to resolve: could not find `rustc_driver` in `self`
2019-09-19T18:06:26.4266525Z --> src/tools/rls/rls-rustc/src/clippy.rs:52:15
2019-09-19T18:06:26.4266824Z |
2019-09-19T18:06:26.4268557Z 52 | use self::rustc_driver::plugin::registry::Registry;
2019-09-19T18:06:26.4269338Z | ^^^^^^^^^^^^ could not find `rustc_driver` in `self`
