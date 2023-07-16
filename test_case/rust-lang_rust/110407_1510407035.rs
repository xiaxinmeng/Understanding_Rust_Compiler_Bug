plain

error: cannot determine resolution for the macro `fluent_messages`
  --> src/lib.rs:90:1
   |
90 | fluent_messages! { "../messages.ftl" }
   |
   |
   = note: import resolution is stuck, try simplifying macro imports
error[E0433]: failed to resolve: could not find `fluent_generated` in the crate root
  --> src/errors.rs:19:10
   |
19 |   #[derive(Diagnostic)]
---
help: consider importing one of these items
    |
65  | use rustc_attr::DEFAULT_LOCALE_RESOURCE;
    |
65  | use rustc_codegen_ssa::DEFAULT_LOCALE_RESOURCE;
65  | use rustc_driver::DEFAULT_LOCALE_RESOURCE;
    |
65  | use rustc_errors::DEFAULT_LOCALE_RESOURCE;
    |
    |
      and 3 other candidates
help: if you import `DEFAULT_LOCALE_RESOURCE`, refer to it directly
109 -         crate::DEFAULT_LOCALE_RESOURCE
109 +         DEFAULT_LOCALE_RESOURCE
    |


error: unused imports: `DiagnosticMessage`, `SubdiagnosticMessage`
   |
   |
78 | use rustc_errors::{DiagnosticMessage, ErrorGuaranteed, Handler, SubdiagnosticMessage};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
Some errors have detailed explanations: E0425, E0432, E0433.
For more information about an error, try `rustc --explain E0425`.
error: could not compile `rustc_codegen_gcc` due to 27 previous errors
Build completed unsuccessfully in 0:01:40
