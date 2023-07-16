plain
    Checking rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
error: unused variable: `infer`
  --> compiler/rustc_trait_selection/src/traits/wf.rs:44:38
   |
44 |                 ty::ConstKind::Infer(infer) => {
   |                                      ^^^^^ help: if this is intentional, prefix it with an underscore: `_infer`
   |
   = note: `-D unused-variables` implied by `-D warnings`
error: could not compile `rustc_trait_selection` due to previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `rustc_trait_selection` due to previous error
Build completed unsuccessfully in 0:02:07
