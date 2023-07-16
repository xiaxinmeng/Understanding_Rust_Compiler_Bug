
error: internal compiler error: expansion entered force mode without producing any errors
 --> core/editor/src/communication/document_action_handler.rs:7:1
  |
7 | #[impl_message(Message, Document)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: delayed at compiler/rustc_expand/src/expand.rs:450:34

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1013:13
stack backtrace:
