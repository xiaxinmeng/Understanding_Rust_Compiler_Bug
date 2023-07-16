
> ---- [ui] src/test/ui/lang-items/issue-83471.rs stdout ----
> diff of stderr:
> 
> 36	   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
> 37	   = note: `#[warn(anonymous_parameters)]` on by default
> 38	
> -	error[E0718]: `fn` language item must be applied to a trait with 1 generic argument
> +	error[E0718]: `r#fn` language item must be applied to a trait with 1 generic argument
> 40	  --> $DIR/issue-83471.rs:11:1
> 41	   |
> 42	LL | #[lang = "fn"]
> 