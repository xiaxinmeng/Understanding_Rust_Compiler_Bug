
let j = a.wrapping_sub(pmoda) >> gcdpow; // same as `a.wrapping_sub(pmoda)`
let k = smoda >> gcdpow; // same as `smoda`
return intrinsics::unchecked_rem(j.wrapping_mul(mod_inv(k, a)), a >> gcdpow); 
