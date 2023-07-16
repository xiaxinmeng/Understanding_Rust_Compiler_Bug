
// --target wasm32-unknown-unknown -C target-feature=+multivalue -O

pub fn read_number(number: &str) -> Result<f32, std::num::ParseFloatError> {
    number.parse()
}
