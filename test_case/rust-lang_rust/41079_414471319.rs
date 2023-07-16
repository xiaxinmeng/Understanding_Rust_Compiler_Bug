rust
let all_code_units_ptr = all_code_units.as_ptr();

//...

let code_unit_index = unsafe { iter.as_slice().as_ptr().offset_from(all_code_units_ptr) } as usize - 1;

//...

let end_code_unit_index = unsafe { slice.as_ptr().offset_from(all_code_units_ptr) } as usize - 1;
