rust
let mut a = Rc::new(5_i32);
let b = Rc::clone(&a);

let violated_ref: &i32 = b.deref();
unsafe {
     *Rc::get_mut_unchecked(&mut a) = 10;
};

println!("{}", *violate_ref);
