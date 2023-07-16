rust

trait PlaceVisitor: MirVisitor { // no MutVisitor equivalent
  fn visit_projection(...);
}

impl<'tcx> MirVisitor<'tcx> for Promoter<'_, 'tcx> {
  delegate_super_place!(self);
  // macro creates:
  fn super_place(self) {
    
    for each projection { self.visit_projection(...); }
  }
}

impl<'a, 'tcx> PlaceVisitor<'tcx> for Promoter<'a, 'tcx> {
}
