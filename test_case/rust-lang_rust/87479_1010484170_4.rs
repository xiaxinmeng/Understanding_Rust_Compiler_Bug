rust
pub trait Machine<'a> {
    type Datum<'b, 'c>
    where
        Self: 'b;
    type Function<'b>: for<'c> Function<Datum<'c> = Self::Datum<'b, 'c>>
    where
        Self: 'b;
}

pub trait Function {
    type Datum<'c>;
    /* … */
}

pub trait Module<'a, 'b>: Sized {
    type Machine: Machine<'a>; // TODO: replace with `type Datum<'c>;`, when compiler is fixed
    fn open(&self, name: &str) -> Result<Self, MachineError>;
    fn get(
        &self,
        name: &str,
    ) -> Result<<Self::Machine as Machine<'a>>::Datum<'b, 'static>, MachineError>;
    fn set<'c>(
        &self,
        name: &str,
        datum: <Self::Machine as Machine<'a>>::Datum<'b, 'c>,
    ) -> Result<(), MachineError>;
}
