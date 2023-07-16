plain
[00:33:54]    Compiling aho-corasick v0.6.4
[00:34:01]    Compiling tempdir v0.3.7
[00:34:35]    Compiling minifier v0.0.11
[00:34:38]    Compiling rustdoc v0.0.0 (file:///checkout/src/librustdoc)
[00:34:46] error[E0063]: missing field `hir_id` in initializer of `rustc::hir::Lifetime`
[00:34:46]    --> librustdoc/clean/auto_trait.rs:260:36
[00:34:46]     |
[00:34:46] 260 |                     lifetimes.push(hir::Lifetime {
[00:34:46]     |                                    ^^^^^^^^^^^^^ missing `hir_id`
