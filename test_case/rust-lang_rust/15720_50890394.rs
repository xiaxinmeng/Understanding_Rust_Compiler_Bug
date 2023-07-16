

pub enum Json {
    Number(f64),
    String(String),
    Boolean(bool),
    List(List),
    Object(Object),
    Null,
}

pub type Object = HashMap<String, Json>;


    fn build_object(&mut self) -> Result<Json, BuilderError> {
        self.bump();

        let mut values = HashMap::new();

        loop {
            match self.token {
                Some(ObjectEnd) => { return Ok(Object(values)); }
                Some(Error(e)) => { return Err(e); }
                None => { break; }
                _ => {}
            }
            let key = match self.parser.stack().top() {
                Some(Key(k)) => { k.to_string() }
                _ => { fail!("invalid state"); }
            };
            match self.build_value() {
                Ok(value) => { values.insert(key, value); }
                Err(e) => { return Err(e); }
            }
            self.bump();
        }
        return self.parser.error(EOFWhileParsingObject);
    }
