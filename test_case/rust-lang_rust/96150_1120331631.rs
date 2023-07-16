plain
   Compiling proc-macro-api v0.0.0 (/checkout/src/tools/rust-analyzer/crates/proc-macro-api)
   Compiling lsp-server v0.6.0
   Compiling lsp-types v0.93.0
   Compiling proc-macro-srv v0.0.0 (/checkout/src/tools/rust-analyzer/crates/proc-macro-srv)
warning: 6th rule of macro `quote_tt` is never used
  --> crates/proc-macro-srv/src/abis/abi_1_56/proc_macro/quote.rs:15:5
   |
15 |     (:) => { Punct::new(':', Spacing::Alone) };
   |
   = note: `#[warn(unused_macro_rules)]` on by default


warning: 6th rule of macro `quote_tt` is never used
  --> crates/proc-macro-srv/src/abis/abi_1_57/proc_macro/quote.rs:15:5
   |
15 |     (:) => { Punct::new(':', Spacing::Alone) };


warning: 6th rule of macro `quote_tt` is never used
  --> crates/proc-macro-srv/src/abis/abi_1_58/proc_macro/quote.rs:15:5
   |
15 |     (:) => { Punct::new(':', Spacing::Alone) };

   Compiling flycheck v0.0.0 (/checkout/src/tools/rust-analyzer/crates/flycheck)
   Compiling project-model v0.0.0 (/checkout/src/tools/rust-analyzer/crates/project-model)
   Compiling hir-ty v0.0.0 (/checkout/src/tools/rust-analyzer/crates/hir-ty)
   Compiling hir-ty v0.0.0 (/checkout/src/tools/rust-analyzer/crates/hir-ty)
   Compiling hir v0.0.0 (/checkout/src/tools/rust-analyzer/crates/hir)
   Compiling ide-db v0.0.0 (/checkout/src/tools/rust-analyzer/crates/ide-db)
   Compiling ide-assists v0.0.0 (/checkout/src/tools/rust-analyzer/crates/ide-assists)
   Compiling ide-diagnostics v0.0.0 (/checkout/src/tools/rust-analyzer/crates/ide-diagnostics)
   Compiling ide-ssr v0.0.0 (/checkout/src/tools/rust-analyzer/crates/ide-ssr)
   Compiling ide-completion v0.0.0 (/checkout/src/tools/rust-analyzer/crates/ide-completion)
warning: 4th rule of macro `attrs` is never used
    |
    |
188 |     [@ { $ty:ident $($tt:tt)* } {$($acc:tt)*}] => {
    |
    = note: `#[warn(unused_macro_rules)]` on by default


warning: 6th rule of macro `attrs` is never used
    |
    |
195 |     [@ {$($tt:tt)+} {$($tt2:tt)*}] => {

   Compiling ide v0.0.0 (/checkout/src/tools/rust-analyzer/crates/ide)
warning: `proc-macro-srv` (lib) generated 3 warnings
warning: `ide-completion` (lib) generated 2 warnings
