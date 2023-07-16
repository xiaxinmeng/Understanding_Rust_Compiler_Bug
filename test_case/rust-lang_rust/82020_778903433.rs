
error[E0500]: closure requires unique access to `self` but it is already borrowed
  --> src/librustdoc/clean/auto_trait.rs:43:25
   |
41 |         let auto_traits = self.cx.auto_traits.iter().cloned();
   |                           ------------------- borrow occurs here
42 |         auto_traits
43 |             .filter_map(|trait_def_id| {
   |              ---------- ^^^^^^^^^^^^^^ closure construction occurs here
   |              |
   |              first borrow later used by call
...
46 |                     substs: self.cx.tcx.mk_substs_trait(ty, &[]),
   |                             ---- second borrow occurs due to use of `self` in closure
