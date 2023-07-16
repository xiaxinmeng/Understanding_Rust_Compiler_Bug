
foo.rs:8:31: 8:43 error: binary operation + cannot be applied to type `~[U]`
foo.rs:8         for x in self.iter() { r += ~[f(x)]; }
                                        ^~~~~~~~~~~~
error: internal compiler error: no type for node 61: expr [f(x)] (id=61) in fcx 7fa3c8c584d0
