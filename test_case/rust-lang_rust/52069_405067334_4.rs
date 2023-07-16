
warning: implicit lifetime parameters in types are deprecated
  --> elide.rs:20:15
   |
20 |     let b2a = Ref::clone(b1);
   |               ^^^-^^^^^^
   |                  |
   |                  help: indicate the anonymous lifetime: `<'_>`
