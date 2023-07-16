rust
    pub fn project_index<Bx: BuilderMethods<'a, 'tcx, Value = V>>(
        &self,
        bx: &mut Bx,
        llindex: V
    ) -> Self {
        let index = if bx.cx().is_const_integral(llindex) { bx.cx().const_to_uint(llindex) } else { 1 };
        let layout = self.layout.field(bx.cx(), 0);
        PlaceRef {
            llval: bx.inbounds_gep(self.llval, &[bx.cx().const_usize(0), llindex]),
            llextra: None,
            layout: layout,
            align: self.align.restrict_for_offset(index * layout.size)
        }
    }
