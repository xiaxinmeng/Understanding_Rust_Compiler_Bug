
warning: to use a constant of type `Small` in a pattern, `Small` must be annotated with `#[derive(PartialEq, Eq)]`, #[warn(illegal_struct_or_enum_constant_pattern)] on by default
  --> <anon>:19:12
19 |>     if let QUX = cake {
   |>            ^^^
warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
note: for more information, see RFC 1445 <https://github.com/rust-lang/rfcs/pull/1445>
