rust
 let id_span_message = (msg_id, span_maybe, message.to_owned());
 let fresh = self.one_time_diagnostics.borrow_mut().insert(id_span_message);
