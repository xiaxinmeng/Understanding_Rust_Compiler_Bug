
error[E0389]: cannot assign through `&`-reference `fancy_ref`
 --> src/main.rs:8:5
  |
7 |    let fancy_ref = &(&mut fancy);
  |                    - help: consider changing this to be a mutable reference: `&mut `
8 |     fancy_ref.num = 6; //~ ERROR E0389
  |     ^^^^^^^^^^^^^^^^^ cannot assign through `&`-reference
