diff
> Abi::ScalarPair { .. }
> +                        if self.tcx.sess.target.arch == "m68k" =>
> +                    {
> +                        arg.make_indirect();
> +                        return;
> +                    }
> 