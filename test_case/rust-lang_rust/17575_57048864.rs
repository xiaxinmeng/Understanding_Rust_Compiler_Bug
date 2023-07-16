
/home/ariel/Rust/safe-ty/src/librustc/middle/trans/reflect.rs:132:19: 132:27 error: cannot infer an appropriate lifetime due to conflicting requirements
/home/ariel/Rust/safe-ty/src/librustc/middle/trans/reflect.rs:132         let bcx = self.bcx;
                                                                                    ^~~~~~~~
/home/ariel/Rust/safe-ty/src/librustc/middle/trans/reflect.rs:131:5: 406:6 note: consider using an explicit lifetime parameter as shown: fn visit_ty(&mut self, t: Ty<'blk>)
/home/ariel/Rust/safe-ty/src/librustc/middle/trans/reflect.rs:131     pub fn visit_ty(&mut self, t: Ty<'tcx>) {
/home/ariel/Rust/safe-ty/src/librustc/middle/trans/reflect.rs:132         let bcx = self.bcx;
/home/ariel/Rust/safe-ty/src/librustc/middle/trans/reflect.rs:133         let tcx = bcx.tcx();
/home/ariel/Rust/safe-ty/src/librustc/middle/trans/reflect.rs:134         debug!("reflect::visit_ty {}", ty_to_string(bcx.tcx(), t));
/home/ariel/Rust/safe-ty/src/librustc/middle/trans/reflect.rs:135 
/home/ariel/Rust/safe-ty/src/librustc/middle/trans/reflect.rs:136         match ty::get(t).sty {
                                                                  ...
