rust
// T: Copy
fn foo(x: &mut T) {  
   // in this scope there is only one 
   // pointer to the value behind x
   let y = *x; 
    ... 
}
{
   let mut z = T;
   let ptr = &mut z as *mut T;
   // hoist the load from foo out here
   foo(&mut z);     
   *ptr = T;
}
