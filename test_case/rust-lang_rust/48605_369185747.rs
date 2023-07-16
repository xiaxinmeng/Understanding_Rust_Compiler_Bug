rust
let mut x = &mut vec![]; 
//  ^^^ not needed
let closure = || {
    x.push(22);
};
closure();
