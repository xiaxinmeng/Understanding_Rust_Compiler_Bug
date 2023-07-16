rust
> #[unleash_the_miri_inside_of_you_here]
> fn const_eval_select<ARG, F, G, RET>(arg: ARG, called_in_const: F, called_at_rt: G) -> RET {
>     if this_is_run_in_const_eval() {
>         called_in_const(arg)
>     } else {
>         called_at_rt(arg)
>     }
> }
> 