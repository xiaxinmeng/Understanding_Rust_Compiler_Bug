rust
    <u8 as Tr(&u8)>::method; // ICE
    <u8 as Tr(&u8) -> ()>::method; // equivalent, ICE
    <u8 as Tr<(&u8,), Output = ()>>::method; // desugared, no ICE
