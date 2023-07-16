rust
read_into(&mut [0u8]);
read_into(&mut [MaybeUninit::uninit()]);
read_into(&mut some_data_structure_that_can_be_read_into);
