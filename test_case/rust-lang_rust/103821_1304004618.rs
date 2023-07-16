rust
fn get_mut_ref<'a>() -> &'a mut () { Box::leak(Box::new(())) }
