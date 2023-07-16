rust
let all_code_units_ptr_as_usize = all_code_units.as_ptr() as usize;

//...

let code_unit_index = unsafe { intrinsics::exact_div(iter.as_slice().as_ptr() as usize - all_code_units_ptr_as_usize, mem::size_of::<CodeUnitT>()) } - 1;

//...

let end_code_unit_index = unsafe { intrinsics::exact_div(slice.as_ptr() as usize - all_code_units_ptr_as_usize, mem::size_of::<CodeUnitT>()) } - 1;
