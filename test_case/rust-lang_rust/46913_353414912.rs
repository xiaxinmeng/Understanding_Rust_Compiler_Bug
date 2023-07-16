
[00:15:32] error[E0507]: cannot move out of borrowed content
[00:15:32]    --> /checkout/src/librustc_metadata/cstore.rs:114:9
[00:15:32]     |
[00:15:32] 114 |         self.metas.borrow().get(cnum).unwrap().unwrap().clone()
[00:15:32]     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot move out of borrowed content
[00:15:32] 
[00:15:32] error[E0596]: cannot borrow immutable local variable `met` as mutable
[00:15:32]    --> /checkout/src/librustc_metadata/cstore.rs:121:13
[00:15:32]     |
[00:15:32] 119 |         let met = self.metas.borrow_mut();
[00:15:32]     |             --- consider changing this to `mut met`
[00:15:32] 120 |         while met.len() <= cnum.index() {
[00:15:32] 121 |             met.push(None);
[00:15:32]     |             ^^^ cannot borrow mutably
[00:15:32] 
[00:15:32] error[E0596]: cannot borrow immutable local variable `met` as mutable
[00:15:32]    --> /checkout/src/librustc_metadata/cstore.rs:123:9
[00:15:32]     |
[00:15:32] 119 |         let met = self.metas.borrow_mut();
[00:15:32]     |             --- consider changing this to `mut met`
[00:15:32] ...
[00:15:32] 123 |         met[cnum] = Some(data);
[00:15:32]     |         ^^^ cannot borrow mutably
[00:15:32] 
