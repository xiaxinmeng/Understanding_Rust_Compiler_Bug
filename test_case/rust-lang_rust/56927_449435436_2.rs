rust
    pub fn project_index<Bx: BuilderMethods<'a, 'tcx, Value = V>>(
        &self,
        bx: &mut Bx,
        llindex: V
    ) -> Self {
        PlaceRef {
            llval: bx.inbounds_gep(self.llval, &[bx.cx().const_usize(0), llindex]),
            llextra: None,
            layout: self.layout.field(bx.cx(), 0),
            align: self.align
        }
    }
