
hi.rs:8:32: 8:51 error: transmute called on types with different sizes: &&() (64 bits) to () (0 bits)
hi.rs:8                 let attr: () = std::mem::transmute(attr);
                                       ^~~~~~~~~~~~~~~~~~~
