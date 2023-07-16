rust
/// Variable register of the CHIP-8 processor
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Step, TryFromPrimitive)]
#[repr(u8)]
pub enum VarRegister {
    /// Used as the offset in [`Instruction::JumpOffset`].
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    VA,
    VB,
    VC,
    VD,
    VE,
    /// Used for carry/borrow flags and set to the shifted-out bit after bit shifts.
    /// See [`Instruction::AddAssign`], [`Instruction::SubAssign`],
    /// [`Instruction::RevSubAssign`], [`Instruction::ShrAssign`]
    /// and [`Instruction::ShlAssign`].
    VF,
}
// -- snip --
        match instruction {
            // -- snip --
            Instruction::StoreRegisterValues { last_register } => {
                for register in VarRegister::V0..=last_register {
                    self.memory[self.address_register as usize + register as u8 as usize] =
                        self.get_register(register);
                }
            }
            Instruction::LoadRegisterValues { last_register } => {
                for register in VarRegister::V0..=last_register {
                    self.set_register(
                        register,
                        self.memory[self.address_register as usize + register as u8 as usize],
                    );
                }
            }
            // -- snip --
        }
// -- snip --
