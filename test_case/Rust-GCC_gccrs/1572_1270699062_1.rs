text
enum Option<T> {
	Some(T),
	None,
}

enum MyCoolEnum<T, U> {
	JustAName,
	Single(T),
	Tupl(T, u32, U),
	Struc {
		a: i16,
		b: ,               <------- Dump::visit(TupleType &) TODO
		c: char,
	},
	Str {},
	Disc = 20,
}

union MyCoolUnion<W, X> {
	a: u64,
	b: W,
	c: MyCoolEnum<Ambiguous: X, Ambiguous: W>,
}

struct MyCoolStruct<T> {
	x: Option<Ambiguous: T>,
	y: MyCoolEnum<Ambiguous: T, Ambiguous: T>,
	z: MyCoolUnion<Ambiguous: u64, Ambiguous: char>,
}

struct CoolTupl<B, C>(B, i16, C);
