rust
// Works fine in playground.

macro_rules! gen_orm {
	($name:ident, $table:ident, $table_name:expr, {$($field:ident: $type:ty,)+}) => ()
}

macro_rules! gen_orm2 {
	($name:ident, $table:ident, {$($field:ident: $type:ty,)+}) => (
		gen_orm!($name, $table, stringify!($table), {$($field: $type,)+});
	)
}

fn main() {
    gen_orm2!(Foo, foos, { meow: i32, percent: i32, });
}
