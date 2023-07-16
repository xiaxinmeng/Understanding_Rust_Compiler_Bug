diff
>  if self.tcx.sess.opts.debugging_opts.unleash_the_miri_inside_of_you {
>      self.tcx.sess.miri_unleashed_feature(span, gate);
>      return;
>  }
> +
> +let attributes = self.tcx.get_attrs(self.def_id().to_def_id());
> +if self.tcx.sess.contains_name(&attributes, sym::lang) {
> +    // Disable const check for lang items
> +    // FIXME: What should be done here?
> +    return;
> +}
>
>  let mut err = op.build_error(self.ccx, span);
>  assert!(err.is_error())
> 