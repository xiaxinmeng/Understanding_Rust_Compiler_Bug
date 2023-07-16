
#[rustc_valid_scalar_range_start(0)]
struct size(isize);
/*safe*/ fn try_size_of_val_raw<T>(*const T) -> Option<size>;
