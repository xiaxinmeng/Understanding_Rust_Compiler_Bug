
let q = carry / rhs;
let r = carry % rhs;

q * NANOS_PER_SEC + r * NANOS_PER_SEC / rhs
