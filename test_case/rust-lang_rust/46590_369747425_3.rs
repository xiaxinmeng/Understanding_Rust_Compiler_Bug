rust
if self.nonlexical_cause_info.is_none() {
    let region_cx = self.nonlexical_regioncx.as_ref().unwrap();
    self.nonlexical_cause_info = Some(region_cx.compute_cause_info());
}
