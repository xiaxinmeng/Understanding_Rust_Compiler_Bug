
trait HashableEncodable: Hashable + Encodable {
    fn hash(&self) -> uint {
         let hash_encoder = HashEncoder::new();
         self.encode(&hash_encoder);
         hash_encoder.hash
    }
}
