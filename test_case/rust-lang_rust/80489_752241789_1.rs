rust
> let attributes = self.tcx.get_attrs(self.def_id().to_def_id());
> if self.tcx.sess.contains_name(&attributes, sym::unleash_the_miri_inside_of_you_here) {
>     // FIXME: What should be done here?
>     println!("Yay I noticed the unleash_the_miri_inside_of_you_here attribute!!!");
>     return;
> }
> 