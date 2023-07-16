rust
let mut a = &mut 5; // line 64 in lint-unused-mut-variables.rs

let mut a = 5; // line 68 in lint-unused-mut-variables.rs
let mut b = (&mut a,); // line 69 in lint-unused-mut-variables.rs

fn mut_ref_arg(mut arg : &mut [u8]) -> &mut [u8] {
    &mut arg[..] // line 80 in lint-unused-mut-variables.rs
}

let mut v : &mut Vec<()> = &mut vec![]; // line 84 in lint-unused-mut-variables.rs
