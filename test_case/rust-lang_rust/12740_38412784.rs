
impl Decodable<json::Decoder> for Json {
    fn decode(d: &mut json::Decoder) -> Result<Json, Error> {
        d.builder.build()
    }
}
