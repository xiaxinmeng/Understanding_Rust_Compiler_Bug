
foo.rs:8:8: 8:30 error: binary operation + cannot be applied to type `T`
foo.rs:8         self.x += v.x.clone();
                 ^~~~~~~~~~~~~~~~~~~~~~
foo.rs:9:8: 9:30 error: binary operation + cannot be applied to type `T`
foo.rs:9         self.y += v.y.clone();
                 ^~~~~~~~~~~~~~~~~~~~~~
foo.rs:10:8: 10:30 error: binary operation + cannot be applied to type `T`
foo.rs:10         self.z += v.z.clone();
                  ^~~~~~~~~~~~~~~~~~~~~~
error: internal compiler error: no type for node 36: callee_scope v.x.clone() (id=36) in fcx 7fd2bc7f4090
