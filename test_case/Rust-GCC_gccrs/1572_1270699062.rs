rust
enum Option<T> {
    Some(T),
    None,
}

enum MyCoolEnum<T, U> {
    JustAName,
    #[doc = "foo"]
    Single (T),
    Tupl (T, u32, U),
    Struc {
        a: i16,
        b: (T, U),
        c: char,
    },
    Str {},

    Disc = 20,
}

union MyCoolUnion<W,X> {
    a: u64,
    b: W,
    c: MyCoolEnum<X,W>,
}

struct MyCoolStruct<T> {
    x: Option<T>,
    y: MyCoolEnum<T,T>,
    z: MyCoolUnion<u64, char>,
}

struct CoolTupl<B,C> (B, i16, C);
