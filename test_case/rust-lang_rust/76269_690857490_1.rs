
fn parameterized_call_fn<F: Fn(u32) -> u32>(f: &F, x: u32) { f(x); }
parameterized_call_fn(&fn_name, 1);
