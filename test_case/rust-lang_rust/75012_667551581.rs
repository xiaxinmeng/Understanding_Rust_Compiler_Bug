rust
impl<'a> struct R8VM<'a> {
    ...
    pub fn call(&mut self, sym: SymID, args: &[PV]) -> Result<SPV<'a>, Error> {
        // The actual code doesn't just say unimplemented, but doing that still causes the bug
        unimplemented!()
    }
    // This addition is what caused the build to start segfaulting
    pub fn call_s(&mut self, name: &str, args: &[PV]) -> Result<SPV<'a>, Error> {
        let sym = self.mem.sym(name); // This line can be removed
        self.call(sym, args)

        // This also causes the segfault
        self.call(SymID { id: 0 }, args)
    }
    ...
}
