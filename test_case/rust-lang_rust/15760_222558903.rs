
warning: this feature is stable. attribute no longer needed, #[warn(stable_features)] on by default
 --> <anon>:1:12
1 |> #![feature(default_type_params)]
  |>            ^^^^^^^^^^^^^^^^^^^

warning: defaults for type parameters are only allowed in `struct`, `enum`, `type`, or `trait` definitions., #[warn(invalid_type_param_default)] on by default
 --> <anon>:3:8
3 |> fn foo<T = u64>(t: T) {}
  |>        ^
warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
note: for more information, see PR 30742 <https://github.com/rust-lang/rust/pull/30724>

warning: unused variable: `t`, #[warn(unused_variables)] on by default
 --> <anon>:3:17
3 |> fn foo<T = u64>(t: T) {}
  |>                 ^
