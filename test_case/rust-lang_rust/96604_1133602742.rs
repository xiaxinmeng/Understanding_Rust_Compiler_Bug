rust
const fn precompute(x: u64) -> Option<u32> {
   return match x {
     1 => // special case
     2 => // special case
     _ => None
   }
}



fn foo(x: u64)  -> u32 {
   let precomputed = const { precompute(optimized_or_default(&x, u64::MAX)) }
   // magic happens here -------------------------------------^
   if let Some(p) = precomputed {
      return p;
   }
   // runtime impl 
}
