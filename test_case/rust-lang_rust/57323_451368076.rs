rust
if let Some(span) = span {
  feature_gate::emit_feature_err(/* ... */);
} else {
  self.tcx.sess.err("some error message here");
}
