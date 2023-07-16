rust
pub struct Value {}
impl Value {
    pub fn as_u32_vec(self) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
        unimplemented!()
    }
}
pub struct TIFFDecoder {}
impl TIFFDecoder {
    fn get_tag(&mut self) -> Result<Value, Box<dyn std::error::Error>> {
        unimplemented!()
    }
    pub fn get_tag_u32_vec(&mut self) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
        (try!(self.get_tag())).as_u32_vec()
    }
}

