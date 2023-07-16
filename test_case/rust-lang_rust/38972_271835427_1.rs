rust
impl Deserialize for Config {
    fn deserialize<D>(deserializer: &mut D) -> Result<Config, D::Error>
        where D: Deserializer
    {
        enum Field {}

        impl Deserialize for Field {
            /* ... */
        }

        struct Visitor<D: Deserializer>(PhantomData<D>);

        impl<D: Deserializer> Visitor for Visitor<D> {
            type Value = Config;
            fn visit_seq<V>(&mut self, mut visitor: V) -> Result<Config, V::Error>
                where V: SeqVisitor
            {
                try!(visitor.end());
                Ok(Config{})
            }
            fn visit_map<V>(&mut self, mut visitor: V) -> Result<Config, V::Error>
                where V: MapVisitor
            {
                // ERROR: irrefutable while-let pattern
                while let Some(key) = try!(visitor.visit_key::<Field>()) {
                    match key {}
                }
                try!(visitor.end());
                Ok(Config{})
            }
        }

        const FIELDS: &'static [&'static str] = &[];
        deserializer.deserialize_struct("Config", FIELDS, Visitor::<D>(PhantomData))
    }
}
