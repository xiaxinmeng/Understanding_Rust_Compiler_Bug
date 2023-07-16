 rust
[].split(|x| x.clone()); // This code causes an ICE, but
[].split(|&x| x.clone()); // This code doesn't cause an ICE
