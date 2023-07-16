rust
impl Parameter {
    pub fn set<T: ParmType>(&self, value: impl AsRef<[<T as ParmType>::Type]>) -> Result<()> {
        T::set(self.node, &self.info, value)
    }
}

// 
let parm: Parameter;
parm.set::<i32>([1,2,3]);
