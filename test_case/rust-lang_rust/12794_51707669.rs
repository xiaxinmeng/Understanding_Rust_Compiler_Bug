 rust
#[deriving(Decodable)]
struct Category {
    min: uint,
    max: uint
}

//...
let mut category = Category { min: 0, max: 5 };
let category = Decodable::decode(&mut decoder, &mut category);
