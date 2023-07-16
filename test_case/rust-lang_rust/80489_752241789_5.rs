rust
>  #[unleash_the_miri_inside_of_you_here(const_eval_select)]
> fn const_eval_select<ARG, F, G, RET>(arg: ARG, called_in_const: F, called_at_rt: G) -> RET {
>     called_at_rt(arg)
> }
> 