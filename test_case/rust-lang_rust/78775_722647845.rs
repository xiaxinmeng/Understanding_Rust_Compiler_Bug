
error: this `#[deprecated]` annotation has no effect
   --> src/tools/rustfmt/src/lib.rs:276:1
    |
276 | #[deprecated(note = "Use FormatReportFormatter instead")]
    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove the unnecessary deprecation attribute
    |
    = note: `#[deny(useless_deprecated)]` on by default
