
error[E0599]: no method named `set_data` found for type `encodings::decoding::PlainDecoder<T>` in the current scope
   --> src/column/reader.rs:443:18
    |
443 |       dictionary.set_data(page.buffer().clone(), num_values as usize)?;
    |                  ^^^^^^^^
    | 
   ::: src/encodings/decoding.rs:83:1
    |
83  | pub struct PlainDecoder<T: DataType> {
    | ------------------------------------ method `set_data` not found for this
    |
    = note: the method `set_data` exists but the following trait bounds were not satisfied:
            `encodings::decoding::PlainDecoder<T> : encodings::decoding::Decoder<_>`
    = help: items from traits can only be used if the trait is implemented and in scope
    = note: the following trait defines an item `set_data`, perhaps you need to implement it:
            candidate #1: `encodings::decoding::Decoder`
    = help: there is a default impl for the trait bound:
            `default impl<T: DataType> Decoder<T> for PlainDecoder<T>`
            A default impl does not count as satisfying the trait bounds; try removing `default`.
