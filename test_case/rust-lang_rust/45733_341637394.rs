rust
macro_rules! gen_orm2 {
	($name:ident, $table:ident, {$($field:ident: $type:ty,)+}) => (
		gen_orm!($name, $table, stringify!($table), {$($field: $type,)+});
	)
}
