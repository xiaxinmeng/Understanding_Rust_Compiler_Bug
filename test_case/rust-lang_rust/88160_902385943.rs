rust
const CONST: isize = 0;

enum Enum {
    Variant0 = CONST,
    Variant1 = CONST + 1,
}

enum Enum256 {
    Variant0 = CONST + 256,
    Variant1 = CONST + 256 + 1,
}

#[repr(isize)]
enum ReprIsizeEnum {
    Variant0 = CONST,
    Variant1 = CONST + 1,
}

fn main(){
    dbg!(std::mem::size_of::<Enum>());
    dbg!(std::mem::size_of::<Enum256>());
    dbg!(std::mem::size_of::<ReprIsizeEnum>());
}
