
// abstract type A<'a>: Debug + 'a;
// abstract type B: for<'a> Fn(&'a u32) -> A<'a>;
// fn closure_hrtb() -> B { |x| x }
