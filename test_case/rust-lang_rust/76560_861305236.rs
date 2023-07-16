rust
⚠ const fn my_cast(arg: i32) -> usize {             
      arg as usize
  }        
                               
⚠ unsafe fn prev<const IMM8: i32, const SIXTEEN: i32>(a: [u8; 16], prev: [u8; 16]) -> [u8; 16]
  where                                                       
⚠     [(); my_cast({ SIXTEEN- IMM8 })]: ,           
  {                                           
      transmute(core::arch::x86_64::_mm_alignr_epi8::<{ SIXTEEN - IMM8 }>(
          transmute(a),                   
          transmute(prev),
      ))
  }                
