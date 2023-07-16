
let mut e = @mut 1;
e = d;  // fine (d from above example)
let mut f = @ 2;
f = d;   // same poor error message as above
